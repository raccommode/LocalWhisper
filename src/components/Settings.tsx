import { useState, useEffect } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { setAutoPaste, setVerbatimMode, setLiveMode, updateHotkey, updateHotkeyPtt, setTtsEnabled, setTtsModel, setTtsRate, updateTtsHotkey, listTtsModels, downloadTtsVoice, downloadPiper, isPiperInstalled, deleteTtsVoice, transcribeFile } from "../lib/commands";
import { onTtsStateChanged, onTtsText, onDownloadProgress, onDownloadComplete } from "../lib/events";
import { useSettings } from "../hooks/useSettings";
import { useAppState } from "../hooks/useAppState";
import { useI18n } from "../lib/i18n";
import type { TtsModelInfo } from "../lib/types";
import { HotkeyPicker } from "./HotkeyPicker";
import { AudioDeviceSelector } from "./AudioDeviceSelector";
import { LanguageSelector } from "./LanguageSelector";
import { ModelSelector } from "./ModelSelector";
import { ModelCatalog } from "./ModelCatalog";
import { UILanguageSwitcher } from "./UILanguageSwitcher";
import { UpdateChecker } from "./UpdateChecker";

export function Settings() {
  const { t } = useI18n();
  const { config, refresh } = useSettings();
  const { isRecording, isTranscribing, lastTranscription, liveText, error } =
    useAppState();
  const [showModels, setShowModels] = useState(false);
  const [ttsModels, setTtsModels] = useState<TtsModelInfo[]>([]);
  const [piperReady, setPiperReady] = useState(false);
  const [isSpeaking, setIsSpeaking] = useState(false);
  const [ttsText, setTtsText] = useState("");
  const [ttsDownloading, setTtsDownloading] = useState<string | null>(null);
  const [ttsDownloadPercent, setTtsDownloadPercent] = useState(0);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);

  const refreshTts = () => {
    listTtsModels().then(setTtsModels).catch(console.error);
    isPiperInstalled().then(setPiperReady).catch(console.error);
  };

  useEffect(() => { refreshTts(); }, []);

  useEffect(() => {
    const unlisteners: (() => void)[] = [];
    onTtsStateChanged((speaking) => {
      setIsSpeaking(speaking);
      if (!speaking) setTtsText("");
    }).then((fn) => unlisteners.push(fn));
    onTtsText((text) => setTtsText(text)).then((fn) => unlisteners.push(fn));
    onDownloadProgress((p) => {
      if (ttsDownloading) setTtsDownloadPercent(Math.round(p.percent));
    }).then((fn) => unlisteners.push(fn));
    onDownloadComplete((id) => {
      if (id === "piper" || ttsModels.some((m) => m.id === id)) {
        setTtsDownloading(null);
        setTtsDownloadPercent(0);
        refreshTts();
      }
    }).then((fn) => unlisteners.push(fn));
    return () => unlisteners.forEach((fn) => fn());
  }, [ttsDownloading]);

  if (!config) {
    return <div className="loading">{t("app.loading")}</div>;
  }

  const handleAutoPaste = async (e: React.ChangeEvent<HTMLInputElement>) => {
    try {
      await setAutoPaste(e.target.checked);
      refresh();
    } catch (err) {
      console.error("Auto-paste change failed:", err);
    }
  };

  return (
    <div className="settings">
      <div className="settings-header">
        <h1>LocalWhisper</h1>
        <div className="status-bar">
          {isRecording && (
            <span className="status recording">{t("settings.recording")}</span>
          )}
          {isTranscribing && (
            <span className="status transcribing">{t("settings.transcribing")}</span>
          )}
          {!isRecording && !isTranscribing && (
            <span className="status idle">{t("settings.ready")}</span>
          )}
        </div>
      </div>

      {error && <div className="error-banner">{error}</div>}

      {config.live_mode && (isRecording || liveText) && (
        <div className="live-transcription">
          <label>{t("settings.liveTranscription")}</label>
          <div className="live-text">
            {liveText || t("settings.liveWaiting")}
          </div>
        </div>
      )}

      {lastTranscription && (
        <div className="last-transcription">
          <label>{t("settings.lastTranscription")}</label>
          <p>{lastTranscription}</p>
        </div>
      )}

      {showModels ? (
        <>
          <div className="section-header">
            <h2>{t("settings.manageModels")}</h2>
            <button
              className="btn btn-secondary btn-sm"
              onClick={() => setShowModels(false)}
            >
              {t("settings.back")}
            </button>
          </div>
          <ModelCatalog />
        </>
      ) : (
        <>
          <div className="settings-section">
            <h2>{t("settings.shortcuts")}</h2>
            <HotkeyPicker
              label={t("settings.hotkeyToggle")}
              currentHotkey={config.hotkey}
              onSave={(hotkey) => updateHotkey(hotkey)}
              onUpdate={refresh}
            />
            <HotkeyPicker
              label={t("settings.hotkeyPtt")}
              currentHotkey={config.hotkey_ptt}
              onSave={(hotkey) => updateHotkeyPtt(hotkey)}
              onUpdate={refresh}
              allowClear
            />
            <p className="help-text" style={{ marginTop: "4px" }}>
              {t("settings.pttHelp")}
            </p>
          </div>

          <div className="settings-section">
            <h2>{t("settings.tts")}</h2>

            {isSpeaking && ttsText && (
              <div className="live-transcription">
                <label>{t("settings.ttsReadingText")}</label>
                <div className="live-text">{ttsText}</div>
              </div>
            )}

            <div className="setting-row">
              <label className="toggle-label">
                <span>
                  {t("settings.ttsEnabled")}
                  {isSpeaking && (
                    <span className="status recording" style={{ marginLeft: "8px", fontSize: "0.8em" }}>
                      {t("settings.ttsSpeaking")}
                    </span>
                  )}
                </span>
                <input
                  type="checkbox"
                  checked={config.tts_enabled}
                  onChange={async (e) => {
                    try {
                      await setTtsEnabled(e.target.checked);
                      refresh();
                    } catch (err) {
                      console.error("TTS toggle failed:", err);
                    }
                  }}
                  className="toggle-input"
                />
                <span className="toggle-switch" />
              </label>
              <p className="help-text">
                {t("settings.ttsEnabledHelp")}
              </p>
            </div>

            {config.tts_enabled && (
              <>
                <HotkeyPicker
                  label={t("settings.ttsHotkey")}
                  currentHotkey={config.tts_hotkey}
                  onSave={(hotkey) => updateTtsHotkey(hotkey)}
                  onUpdate={refresh}
                  allowClear
                />
                <p className="help-text" style={{ marginTop: "4px" }}>
                  {t("settings.ttsHotkeyHelp")}
                </p>

                {/* Piper engine install */}
                <div className="setting-row">
                  {piperReady ? (
                    <p className="help-text" style={{ color: "var(--color-success, #4caf50)" }}>
                      {t("settings.ttsPiperInstalled")}
                    </p>
                  ) : (
                    <>
                      <button
                        className="btn btn-primary"
                        disabled={ttsDownloading === "piper"}
                        onClick={async () => {
                          setTtsDownloading("piper");
                          try { await downloadPiper(); } catch (err) { console.error(err); }
                          setTtsDownloading(null);
                          refreshTts();
                        }}
                      >
                        {ttsDownloading === "piper"
                          ? `${t("wizard.downloading")} ${ttsDownloadPercent}%`
                          : t("settings.ttsInstallPiper")}
                      </button>
                      <p className="help-text">{t("settings.ttsInstallPiperHelp")}</p>
                    </>
                  )}
                </div>

                {/* Speed slider */}
                <div className="setting-row">
                  <label>
                    {t("settings.ttsRate")} — {t("settings.ttsWordsPerMin", { rate: config.tts_rate })}
                  </label>
                  <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                    <span style={{ fontSize: "0.8em", opacity: 0.7 }}>{t("settings.ttsRateSlow")}</span>
                    <input
                      type="range"
                      min={80}
                      max={300}
                      step={10}
                      value={config.tts_rate}
                      onChange={async (e) => {
                        try {
                          await setTtsRate(Number(e.target.value));
                          refresh();
                        } catch (err) {
                          console.error("TTS rate change failed:", err);
                        }
                      }}
                      style={{ flex: 1 }}
                    />
                    <span style={{ fontSize: "0.8em", opacity: 0.7 }}>{t("settings.ttsRateFast")}</span>
                  </div>
                </div>

                {/* Voice catalog */}
                <div className="setting-row">
                  <label>{t("settings.ttsAvailableVoices")}</label>
                  {ttsModels.map((m) => (
                    <div
                      key={m.id}
                      style={{
                        display: "flex",
                        alignItems: "center",
                        justifyContent: "space-between",
                        padding: "8px 0",
                        borderBottom: "1px solid var(--color-border, #333)",
                      }}
                    >
                      <div>
                        <strong>{m.name}</strong>
                        <span style={{ opacity: 0.6, marginLeft: "8px", fontSize: "0.85em" }}>
                          {m.language} — {m.size_label}
                        </span>
                      </div>
                      <div style={{ display: "flex", gap: "6px" }}>
                        {m.is_downloaded ? (
                          <>
                            {config.tts_model === m.id ? (
                              <span className="status idle" style={{ fontSize: "0.85em" }}>
                                {t("settings.ttsActive")}
                              </span>
                            ) : (
                              <button
                                className="btn btn-secondary btn-sm"
                                onClick={async () => {
                                  await setTtsModel(m.id);
                                  refresh();
                                }}
                              >
                                {t("settings.ttsUse")}
                              </button>
                            )}
                            <button
                              className="btn btn-secondary btn-sm"
                              onClick={async () => {
                                await deleteTtsVoice(m.id);
                                refreshTts();
                              }}
                            >
                              {t("settings.ttsDelete")}
                            </button>
                          </>
                        ) : (
                          <button
                            className="btn btn-primary btn-sm"
                            disabled={ttsDownloading === m.id || !piperReady}
                            onClick={async () => {
                              setTtsDownloading(m.id);
                              try { await downloadTtsVoice(m.id); } catch (err) { console.error(err); }
                              setTtsDownloading(null);
                              refreshTts();
                            }}
                          >
                            {ttsDownloading === m.id
                              ? `${ttsDownloadPercent}%`
                              : `${t("settings.ttsInstall")} (${m.size_label})`}
                          </button>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
              </>
            )}
          </div>

          <div className="settings-section">
            <h2>{t("settings.general")}</h2>
            <div className="setting-row">
              <label className="toggle-label">
                <span>{t("settings.autoPaste")}</span>
                <input
                  type="checkbox"
                  checked={config.auto_paste}
                  onChange={handleAutoPaste}
                  className="toggle-input"
                />
                <span className="toggle-switch" />
              </label>
              <p className="help-text">
                {t("settings.autoPasteHelp")}
              </p>
            </div>
          </div>

          <div className="settings-section">
            <h2>{t("settings.fileUpload")}</h2>
            <div className="setting-row">
              <div style={{ display: "flex", gap: "8px", alignItems: "center" }}>
                <button
                  className="btn btn-secondary"
                  onClick={async () => {
                    const file = await open({
                      multiple: false,
                      filters: [
                        {
                          name: "Audio",
                          extensions: ["wav", "mp3", "flac", "ogg"],
                        },
                      ],
                    });
                    if (file) setSelectedFile(file);
                  }}
                >
                  {t("settings.selectFile")}
                </button>
                <button
                  className="btn btn-primary"
                  disabled={!selectedFile || isTranscribing}
                  onClick={async () => {
                    if (!selectedFile) return;
                    try {
                      await transcribeFile(selectedFile);
                    } catch (err) {
                      console.error("File transcription failed:", err);
                    }
                    setSelectedFile(null);
                  }}
                >
                  {isTranscribing
                    ? t("settings.transcribing")
                    : t("settings.transcribeFile")}
                </button>
              </div>
              <p className="help-text" style={{ marginTop: "6px" }}>
                {selectedFile
                  ? selectedFile.split("/").pop()?.split("\\").pop()
                  : t("settings.noFileSelected")}
              </p>
              <p className="help-text">{t("settings.fileUploadHelp")}</p>
            </div>
          </div>

          <div className="settings-section">
            <h2>{t("settings.transcription")}</h2>
            <ModelSelector
              currentModel={config.active_model}
              onUpdate={refresh}
            />
            <LanguageSelector
              currentLanguage={config.language}
              onUpdate={refresh}
            />
            <div className="setting-row">
              <label className="toggle-label">
                <span>{t("settings.verbatimMode")}</span>
                <input
                  type="checkbox"
                  checked={config.verbatim_mode}
                  onChange={async (e) => {
                    try {
                      await setVerbatimMode(e.target.checked);
                      refresh();
                    } catch (err) {
                      console.error("Verbatim mode change failed:", err);
                    }
                  }}
                  className="toggle-input"
                />
                <span className="toggle-switch" />
              </label>
              <p className="help-text">
                {t("settings.verbatimModeHelp")}
              </p>
            </div>
            <div className="setting-row">
              <label className="toggle-label">
                <span>{t("settings.liveMode")}</span>
                <input
                  type="checkbox"
                  checked={config.live_mode}
                  onChange={async (e) => {
                    try {
                      await setLiveMode(e.target.checked);
                      refresh();
                    } catch (err) {
                      console.error("Live mode change failed:", err);
                    }
                  }}
                  className="toggle-input"
                />
                <span className="toggle-switch" />
              </label>
              <p className="help-text">
                {t("settings.liveModeHelp")}
              </p>
            </div>
          </div>

          <div className="settings-section">
            <h2>{t("settings.audio")}</h2>
            <AudioDeviceSelector
              currentDevice={config.audio_device}
              onUpdate={refresh}
            />
          </div>

          <div className="settings-section">
            <h2>{t("settings.interface")}</h2>
            <UILanguageSwitcher />
          </div>

          <div className="settings-section">
            <button
              className="btn btn-secondary"
              onClick={() => setShowModels(true)}
            >
              {t("settings.manageModels")}
            </button>
          </div>

          <div className="settings-section">
            <h2>{t("update.title")}</h2>
            <UpdateChecker />
          </div>
        </>
      )}
    </div>
  );
}
