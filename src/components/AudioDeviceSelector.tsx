import { useEffect, useRef, useState } from "react";
import { listAudioDevices, setAudioDevice, testMicrophone } from "../lib/commands";
import { onMicTestLevel } from "../lib/events";
import { useI18n } from "../lib/i18n";
import type { AudioDevice } from "../lib/types";

type TestState = "idle" | "testing" | "success" | "noSound" | "error";

interface Props {
  currentDevice: string | null;
  onUpdate: () => void;
}

export function AudioDeviceSelector({ currentDevice, onUpdate }: Props) {
  const { t } = useI18n();
  const [devices, setDevices] = useState<AudioDevice[]>([]);
  const [testState, setTestState] = useState<TestState>("idle");
  const [level, setLevel] = useState(0);
  const [errorMsg, setErrorMsg] = useState("");
  const maxLevelRef = useRef(0);

  useEffect(() => {
    listAudioDevices()
      .then(setDevices)
      .catch((e) => console.error("Audio device list failed:", e));
  }, []);

  const handleChange = async (e: React.ChangeEvent<HTMLSelectElement>) => {
    try {
      await setAudioDevice(e.target.value || null);
      onUpdate();
    } catch (err) {
      console.error("Audio device change failed:", err);
    }
  };

  const handleTest = async () => {
    setTestState("testing");
    setLevel(0);
    setErrorMsg("");
    maxLevelRef.current = 0;

    const unlisten = await onMicTestLevel((lvl) => {
      setLevel(lvl);
      if (lvl > maxLevelRef.current) {
        maxLevelRef.current = lvl;
      }
    });

    try {
      await testMicrophone();
      unlisten();
      if (maxLevelRef.current > 0.02) {
        setTestState("success");
      } else {
        setTestState("noSound");
      }
    } catch (err) {
      unlisten();
      setErrorMsg(String(err));
      setTestState("error");
    }

    setTimeout(() => {
      setTestState("idle");
      setLevel(0);
    }, 4000);
  };

  return (
    <div className="setting-row">
      <label>{t("audio.microphone")}</label>
      <select
        value={currentDevice ?? ""}
        onChange={handleChange}
        className="select-input"
      >
        <option value="">{t("audio.default")}</option>
        {devices.map((d) => (
          <option key={d.name} value={d.name}>
            {d.name} {d.is_default ? t("audio.defaultSuffix") : ""}
          </option>
        ))}
      </select>

      <div className="mic-test-row">
        <button
          className="btn btn-sm btn-secondary"
          onClick={handleTest}
          disabled={testState === "testing"}
        >
          {testState === "testing" ? t("audio.testing") : t("audio.testMic")}
        </button>

        {testState === "testing" && (
          <div className="mic-level-bar">
            <div
              className="mic-level-fill"
              style={{ width: `${Math.round(level * 100)}%` }}
            />
          </div>
        )}
      </div>

      {testState === "success" && (
        <div className="mic-test-result success">{t("audio.micOk")}</div>
      )}
      {testState === "noSound" && (
        <div className="mic-test-result no-sound">{t("audio.noSound")}</div>
      )}
      {testState === "error" && (
        <div className="mic-test-result error">{errorMsg || t("audio.error")}</div>
      )}
    </div>
  );
}
