import { useState } from "react";
import { downloadModel, deleteModel } from "../lib/commands";
import { useI18n } from "../lib/i18n";
import { DownloadProgress } from "./DownloadProgress";
import type { ModelInfo } from "../lib/types";

interface Props {
  model: ModelInfo;
  isRecommended: boolean;
  onAction: () => void;
}

export function ModelCard({ model, isRecommended, onAction }: Props) {
  const { t } = useI18n();
  const [downloading, setDownloading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleDownload = async () => {
    setDownloading(true);
    setError(null);
    try {
      await downloadModel(model.id);
      onAction();
    } catch (e) {
      setError(String(e));
    } finally {
      setDownloading(false);
    }
  };

  const handleDelete = async () => {
    try {
      await deleteModel(model.id);
      onAction();
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <div className={`model-card ${isRecommended ? "recommended" : ""}`}>
      <div className="model-info">
        <div className="model-header">
          <span className="model-name">{model.name}</span>
          {isRecommended && (
            <span className="badge recommended-badge">{t("wizard.recommended")}</span>
          )}
          {model.is_english_only && (
            <span className="badge english-badge">EN</span>
          )}
          {model.is_quantized && (
            <span className="badge quantized-badge">Q</span>
          )}
        </div>
        <span className="model-size">{model.size_label}</span>
      </div>

      <div className="model-actions">
        {model.is_downloaded ? (
          <button className="btn btn-danger btn-sm" onClick={handleDelete}>
            {t("modelCard.delete")}
          </button>
        ) : downloading ? (
          <DownloadProgress modelId={model.id} />
        ) : (
          <button className="btn btn-primary btn-sm" onClick={handleDownload}>
            {t("modelCard.install")}
          </button>
        )}
      </div>

      {error && <div className="error-text">{error}</div>}
    </div>
  );
}
