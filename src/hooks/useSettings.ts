import { useEffect, useState } from "react";
import { getConfig } from "../lib/commands";
import type { AppConfig } from "../lib/types";

export function useSettings() {
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [loading, setLoading] = useState(true);

  const refresh = async () => {
    try {
      setConfig(await getConfig());
    } catch (e) {
      console.error("Chargement config impossible :", e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    refresh();
  }, []);

  return { config, loading, refresh };
}
