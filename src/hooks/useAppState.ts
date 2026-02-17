import { useEffect, useState } from "react";
import {
  onRecordingStateChanged,
  onTranscriptionStarted,
  onTranscriptionComplete,
  onError,
} from "../lib/events";

export function useAppState() {
  const [isRecording, setIsRecording] = useState(false);
  const [isTranscribing, setIsTranscribing] = useState(false);
  const [lastTranscription, setLastTranscription] = useState<string | null>(
    null,
  );
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const unlisteners = [
      onRecordingStateChanged(setIsRecording),
      onTranscriptionStarted(() => setIsTranscribing(true)),
      onTranscriptionComplete((text) => {
        setIsTranscribing(false);
        setLastTranscription(text);
      }),
      onError((err) => {
        setIsTranscribing(false);
        setError(err);
        setTimeout(() => setError(null), 5000);
      }),
    ];

    return () => {
      unlisteners.forEach((p) => p.then((fn) => fn()));
    };
  }, []);

  return { isRecording, isTranscribing, lastTranscription, error };
}
