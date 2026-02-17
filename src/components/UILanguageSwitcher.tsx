import { useI18n, type UILocale } from "../lib/i18n";

export function UILanguageSwitcher() {
  const { locale, setLocale, t } = useI18n();

  return (
    <div className="setting-row">
      <label>{t("settings.uiLanguage")}</label>
      <select
        value={locale}
        onChange={(e) => setLocale(e.target.value as UILocale)}
        className="select-input"
      >
        <option value="en">English</option>
        <option value="fr">Francais</option>
      </select>
    </div>
  );
}
