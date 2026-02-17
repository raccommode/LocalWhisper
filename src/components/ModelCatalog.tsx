import { useEffect, useState } from "react";
import { getSystemInfo } from "../lib/commands";
import { useModels } from "../hooks/useModels";
import { useI18n } from "../lib/i18n";
import { ModelCard } from "./ModelCard";
import type { SystemInfo } from "../lib/types";

export function ModelCatalog() {
  const { t } = useI18n();
  const { models, refresh } = useModels();
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);

  useEffect(() => {
    getSystemInfo().then(setSystemInfo);
  }, []);

  const installed = models.filter((m) => m.is_downloaded);
  const available = models.filter((m) => !m.is_downloaded);

  return (
    <div className="model-catalog">
      {systemInfo && (
        <div className="system-info-banner">
          <p>
            {t("catalog.systemInfo", {
              os: systemInfo.os,
              arch: systemInfo.arch,
              ram: systemInfo.total_ram_gb,
              cores: systemInfo.cpu_cores,
            })}
          </p>
          <p className="recommendation">
            {systemInfo.recommended_model_reason}
          </p>
        </div>
      )}

      {installed.length > 0 && (
        <div className="model-section">
          <h3>{t("catalog.installed")}</h3>
          {installed.map((m) => (
            <ModelCard
              key={m.id}
              model={m}
              isRecommended={systemInfo?.recommended_model === m.id}
              onAction={refresh}
            />
          ))}
        </div>
      )}

      <div className="model-section">
        <h3>{t("catalog.available")}</h3>
        {available.map((m) => (
          <ModelCard
            key={m.id}
            model={m}
            isRecommended={systemInfo?.recommended_model === m.id}
            onAction={refresh}
          />
        ))}
      </div>
    </div>
  );
}
