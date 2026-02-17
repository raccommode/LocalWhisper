import { useEffect, useState, useCallback } from "react";
import { listModels } from "../lib/commands";
import { onDownloadComplete } from "../lib/events";
import type { ModelInfo } from "../lib/types";

export function useModels() {
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [loading, setLoading] = useState(true);

  const refresh = useCallback(async () => {
    try {
      setModels(await listModels());
    } catch (e) {
      console.error("Chargement modèles impossible :", e);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    refresh();
    const unlisten = onDownloadComplete(() => refresh());
    return () => {
      unlisten.then((fn) => fn());
    };
  }, [refresh]);

  return { models, loading, refresh };
}
