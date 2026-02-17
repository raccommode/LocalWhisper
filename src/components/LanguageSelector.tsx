import { setLanguage } from "../lib/commands";
import { useI18n } from "../lib/i18n";

const LANGUAGE_CODES = [
  "auto", "fr", "en", "es", "de", "it", "pt", "nl", "ja", "zh", "ko", "ru", "ar", "pl", "uk", "tr",
] as const;

interface Props {
  currentLanguage: string;
  onUpdate: () => void;
}

export function LanguageSelector({ currentLanguage, onUpdate }: Props) {
  const { t } = useI18n();

  const handleChange = async (e: React.ChangeEvent<HTMLSelectElement>) => {
    try {
      await setLanguage(e.target.value);
      onUpdate();
    } catch (err) {
      console.error("Language change failed:", err);
    }
  };

  return (
    <div className="setting-row">
      <label>{t("langSelector.label")}</label>
      <select
        value={currentLanguage}
        onChange={handleChange}
        className="select-input"
      >
        {LANGUAGE_CODES.map((code) => (
          <option key={code} value={code}>
            {t(`langSelector.${code}` as any)}
          </option>
        ))}
      </select>
    </div>
  );
}
