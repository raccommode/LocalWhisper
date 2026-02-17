import { useEffect, useState } from "react";
import {
  checkPermissions,
  requestMicrophonePermission,
  openAccessibilityPreferences,
} from "../lib/commands";
import { useI18n } from "../lib/i18n";
import type { PermissionStatus } from "../lib/types";

interface Props {
  onAllGranted: () => void;
}

export function PermissionSetup({ onAllGranted }: Props) {
  const { t } = useI18n();
  const [status, setStatus] = useState<PermissionStatus | null>(null);
  const [requestingMic, setRequestingMic] = useState(false);

  const refresh = async () => {
    const s = await checkPermissions();
    setStatus(s);
    if (s.microphone && s.accessibility) {
      onAllGranted();
    }
  };

  useEffect(() => {
    refresh();
    const interval = setInterval(refresh, 2000);
    return () => clearInterval(interval);
  }, []);

  const handleRequestMic = async () => {
    setRequestingMic(true);
    await requestMicrophonePermission();
    setTimeout(async () => {
      await refresh();
      setRequestingMic(false);
    }, 1000);
  };

  const handleOpenAccessibility = async () => {
    await openAccessibilityPreferences();
  };

  if (!status) {
    return <div className="loading">{t("permissions.checking")}</div>;
  }

  return (
    <div className="permission-setup">
      <div className="permission-header">
        <h1>{t("permissions.required")}</h1>
        <p>{t("permissions.description")}</p>
      </div>

      <div className="permission-list">
        <div
          className={`permission-item ${status.microphone ? "granted" : ""}`}
        >
          <div className="permission-icon">
            {status.microphone ? (
              <span className="check">&#10003;</span>
            ) : (
              <span className="pending">1</span>
            )}
          </div>
          <div className="permission-info">
            <h3>{t("permissions.microphone")}</h3>
            <p>{t("permissions.microphoneDesc")}</p>
          </div>
          <div className="permission-action">
            {status.microphone ? (
              <span className="granted-label">{t("permissions.granted")}</span>
            ) : (
              <button
                className="btn btn-primary"
                onClick={handleRequestMic}
                disabled={requestingMic}
              >
                {requestingMic ? "..." : t("permissions.allow")}
              </button>
            )}
          </div>
        </div>

        <div
          className={`permission-item ${status.accessibility ? "granted" : ""}`}
        >
          <div className="permission-icon">
            {status.accessibility ? (
              <span className="check">&#10003;</span>
            ) : (
              <span className="pending">2</span>
            )}
          </div>
          <div className="permission-info">
            <h3>{t("permissions.accessibility")}</h3>
            <p>{t("permissions.accessibilityDesc")}</p>
          </div>
          <div className="permission-action">
            {status.accessibility ? (
              <span className="granted-label">{t("permissions.granted")}</span>
            ) : (
              <button
                className="btn btn-primary"
                onClick={handleOpenAccessibility}
              >
                {t("permissions.openSettings")}
              </button>
            )}
          </div>
        </div>
      </div>

      {!status.accessibility && (
        <div className="permission-help">
          <p>{t("permissions.accessibilityHelp")}</p>
        </div>
      )}

      <div style={{ marginTop: "20px", textAlign: "center" }}>
        <button
          className="btn btn-secondary"
          onClick={onAllGranted}
        >
          {t("permissions.skipButton")}
        </button>
        <p className="help-text" style={{ marginTop: "6px" }}>
          {t("permissions.skipHelp")}
        </p>
      </div>
    </div>
  );
}
