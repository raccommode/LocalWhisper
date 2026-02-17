import { useState, useEffect } from "react";
import { check, Update } from "@tauri-apps/plugin-updater";
import { getVersion } from "@tauri-apps/api/app";
import { relaunch } from "@tauri-apps/plugin-process";
import { useI18n } from "../lib/i18n";

export function UpdateChecker() {
  const { t } = useI18n();
  const [currentVersion, setCurrentVersion] = useState<string>("");
  const [update, setUpdate] = useState<Update | null>(null);
  const [checking, setChecking] = useState(false);
  const [downloading, setDownloading] = useState(false);
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getVersion().then(setCurrentVersion).catch(() => {});
  }, []);

  const checkForUpdates = async () => {
    setChecking(true);
    setError(null);
    try {
      const result = await check();
      setUpdate(result);
    } catch (err) {
      setError(String(err));
    } finally {
      setChecking(false);
    }
  };

  const installUpdate = async () => {
    if (!update) return;
    setDownloading(true);
    setError(null);
    try {
      let totalBytes = 0;
      let downloadedBytes = 0;
      await update.downloadAndInstall((event) => {
        if (event.event === "Started" && event.data.contentLength) {
          totalBytes = event.data.contentLength;
        } else if (event.event === "Progress") {
          downloadedBytes += event.data.chunkLength;
          if (totalBytes > 0) {
            setProgress(Math.round((downloadedBytes / totalBytes) * 100));
          }
        } else if (event.event === "Finished") {
          setProgress(100);
        }
      });
      await relaunch();
    } catch (err) {
      setError(String(err));
      setDownloading(false);
    }
  };

  return (
    <div className="update-checker">
      <div className="update-row">
        <div className="update-info">
          <span className="update-version">v{currentVersion}</span>
          {update && (
            <span className="update-available">
              → v{update.version}
            </span>
          )}
          {!update && !checking && !error && (
            <span className="update-status-text">{t("update.upToDate")}</span>
          )}
        </div>
        <div className="update-actions">
          {!update && !downloading && (
            <button
              className="btn btn-sm btn-secondary"
              onClick={checkForUpdates}
              disabled={checking}
            >
              {checking ? t("update.checking") : t("update.checkButton")}
            </button>
          )}
          {update && !downloading && (
            <button
              className="btn btn-sm btn-primary"
              onClick={installUpdate}
            >
              {t("update.installButton")}
            </button>
          )}
        </div>
      </div>
      {update && update.body && !downloading && (
        <div className="update-notes">
          <label>{t("update.releaseNotes")}</label>
          <p>{update.body}</p>
        </div>
      )}
      {downloading && (
        <div className="download-progress">
          <div className="progress-bar">
            <div className="progress-fill" style={{ width: `${progress}%` }} />
          </div>
          <span className="progress-text">{t("update.installing")} {progress}%</span>
        </div>
      )}
      {error && <div className="error-text">{error}</div>}
    </div>
  );
}
