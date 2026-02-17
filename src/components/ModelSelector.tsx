import { loadModel } from "../lib/commands";
import { useModels } from "../hooks/useModels";
import { useI18n } from "../lib/i18n";

interface Props {
  currentModel: string | null;
  onUpdate: () => void;
}

export function ModelSelector({ currentModel, onUpdate }: Props) {
  const { t } = useI18n();
  const { models } = useModels();
  const downloaded = models.filter((m) => m.is_downloaded);

  const handleChange = async (e: React.ChangeEvent<HTMLSelectElement>) => {
    const id = e.target.value;
    if (!id) return;
    try {
      await loadModel(id);
      onUpdate();
    } catch (err) {
      console.error("Model load failed:", err);
    }
  };

  return (
    <div className="setting-row">
      <label>{t("modelSelector.label")}</label>
      <select
        value={currentModel ?? ""}
        onChange={handleChange}
        className="select-input"
      >
        <option value="" disabled>
          {t("modelSelector.placeholder")}
        </option>
        {downloaded.map((m) => (
          <option key={m.id} value={m.id}>
            {m.name} ({m.size_label})
          </option>
        ))}
      </select>
      {downloaded.length === 0 && (
        <p className="help-text">
          {t("modelSelector.noModels")}
        </p>
      )}
    </div>
  );
}
