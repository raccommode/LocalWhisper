import { useEffect, useState } from "react";
import {
  getSystemInfo,
  downloadModel,
  loadModel,
  markSetupComplete,
} from "../lib/commands";
import { useModels } from "../hooks/useModels";
import { useI18n } from "../lib/i18n";
import { DownloadProgress } from "./DownloadProgress";
import type { SystemInfo } from "../lib/types";

interface Props {
  onComplete: () => void;
}

export function SetupWizard({ onComplete }: Props) {
  const { t } = useI18n();
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [selectedModel, setSelectedModel] = useState<string | null>(null);
  const [step, setStep] = useState<"select" | "downloading" | "done">(
    "select",
  );
  const [error, setError] = useState<string | null>(null);
  const { models, refresh } = useModels();

  useEffect(() => {
    getSystemInfo().then((info) => {
      setSystemInfo(info);
      setSelectedModel(info.recommended_model);
    });
  }, []);

  const handleInstall = async () => {
    if (!selectedModel) return;
    setStep("downloading");
    setError(null);

    try {
      await downloadModel(selectedModel);
      await loadModel(selectedModel);
      await markSetupComplete();
      await refresh();
      setStep("done");
      setTimeout(onComplete, 1000);
    } catch (e) {
      setError(String(e));
      setStep("select");
    }
  };

  const mainModels = models.filter(
    (m) => !m.is_quantized && !m.is_english_only,
  );

  return (
    <div className="setup-wizard">
      <div className="wizard-header">
        <h1>{t("wizard.welcome")}</h1>
        <p>{t("wizard.subtitle")}</p>
      </div>

      {systemInfo && (
        <div className="system-info-banner">
          <h3>{t("wizard.yourMachine")}</h3>
          <p>
            {t("wizard.ramCores", {
              ram: systemInfo.total_ram_gb,
              cores: systemInfo.cpu_cores,
              os: systemInfo.os,
            })}
          </p>
        </div>
      )}

      {step === "select" && (
        <>
          <div className="wizard-content">
            <h2>{t("wizard.chooseModel")}</h2>
            {systemInfo && (
              <p className="recommendation">
                {systemInfo.recommended_model_reason}
              </p>
            )}

            <div className="model-selection">
              {mainModels.map((model) => (
                <label
                  key={model.id}
                  className={`model-option ${selectedModel === model.id ? "selected" : ""} ${systemInfo?.recommended_model === model.id ? "recommended" : ""}`}
                >
                  <input
                    type="radio"
                    name="model"
                    value={model.id}
                    checked={selectedModel === model.id}
                    onChange={() => setSelectedModel(model.id)}
                  />
                  <div className="model-option-info">
                    <span className="model-name">{model.name}</span>
                    <span className="model-size">{model.size_label}</span>
                    {systemInfo?.recommended_model === model.id && (
                      <span className="badge recommended-badge">
                        {t("wizard.recommended")}
                      </span>
                    )}
                  </div>
                </label>
              ))}
            </div>

            {error && <div className="error-text">{error}</div>}
          </div>

          <div className="wizard-footer">
            <button
              className="btn btn-primary"
              onClick={handleInstall}
              disabled={!selectedModel}
            >
              {t("wizard.installContinue")}
            </button>
          </div>
        </>
      )}

      {step === "downloading" && selectedModel && (
        <div className="wizard-content center">
          <h2>{t("wizard.downloading")}</h2>
          <p>{t("wizard.downloadHelp")}</p>
          <DownloadProgress modelId={selectedModel} />
        </div>
      )}

      {step === "done" && (
        <div className="wizard-content center">
          <h2>{t("wizard.ready")}</h2>
          <p>{t("wizard.readyMessage")}</p>
        </div>
      )}
    </div>
  );
}
