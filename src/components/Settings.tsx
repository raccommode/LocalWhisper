import { useState } from "react";
import { setAutoPaste, updateHotkey, updateHotkeyPtt } from "../lib/commands";
import { useSettings } from "../hooks/useSettings";
import { useAppState } from "../hooks/useAppState";
import { useI18n } from "../lib/i18n";
import { HotkeyPicker } from "./HotkeyPicker";
import { AudioDeviceSelector } from "./AudioDeviceSelector";
import { LanguageSelector } from "./LanguageSelector";
import { ModelSelector } from "./ModelSelector";
import { ModelCatalog } from "./ModelCatalog";
import { UILanguageSwitcher } from "./UILanguageSwitcher";

export function Settings() {
  const { t } = useI18n();
  const { config, refresh } = useSettings();
  const { isRecording, isTranscribing, lastTranscription, error } =
    useAppState();
  const [showModels, setShowModels] = useState(false);

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
            <h2>{t("settings.transcription")}</h2>
            <ModelSelector
              currentModel={config.active_model}
              onUpdate={refresh}
            />
            <LanguageSelector
              currentLanguage={config.language}
              onUpdate={refresh}
            />
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
        </>
      )}
    </div>
  );
}
