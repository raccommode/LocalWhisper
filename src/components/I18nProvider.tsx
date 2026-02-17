import { useEffect, useState, useCallback, type ReactNode } from "react";
import { I18nContext, t as translate, type UILocale, type TranslationKey } from "../lib/i18n";
import { getConfig, setUiLocale } from "../lib/commands";

export function I18nProvider({ children }: { children: ReactNode }) {
  const [locale, setLocaleState] = useState<UILocale>("en");
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    getConfig().then((config) => {
      const saved = config.ui_locale;
      if (saved === "fr" || saved === "en") {
        setLocaleState(saved);
      }
      setLoaded(true);
    });
  }, []);

  const setLocale = useCallback((newLocale: UILocale) => {
    setLocaleState(newLocale);
    setUiLocale(newLocale).catch(console.error);
  }, []);

  const t = useCallback(
    (key: TranslationKey, params?: Record<string, string | number>) =>
      translate(key, locale, params),
    [locale],
  );

  if (!loaded) return null;

  return (
    <I18nContext.Provider value={{ locale, setLocale, t }}>
      {children}
    </I18nContext.Provider>
  );
}
