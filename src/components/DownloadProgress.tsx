import { useEffect, useState } from "react";
import { onDownloadProgress } from "../lib/events";
import { useI18n } from "../lib/i18n";
import type { DownloadProgress as ProgressData } from "../lib/types";

interface Props {
  modelId: string;
}

export function DownloadProgress({ modelId }: Props) {
  const { t } = useI18n();
  const [progress, setProgress] = useState<ProgressData | null>(null);

  useEffect(() => {
    const unlisten = onDownloadProgress((p) => {
      if (p.model_id === modelId) setProgress(p);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [modelId]);

  if (!progress) return null;

  const pct = Math.round(progress.percent);
  const dlMB = (progress.downloaded_bytes / 1_000_000).toFixed(1);
  const totalMB = (progress.total_bytes / 1_000_000).toFixed(1);
  const unit = t("download.unit");

  return (
    <div className="download-progress">
      <div className="progress-bar">
        <div className="progress-fill" style={{ width: `${pct}%` }} />
      </div>
      <span className="progress-text">
        {dlMB} / {totalMB} {unit} ({pct}%)
      </span>
    </div>
  );
}
