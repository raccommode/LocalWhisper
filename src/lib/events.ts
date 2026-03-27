import { listen } from "@tauri-apps/api/event";
import type { DownloadProgress } from "./types";

export const onRecordingStateChanged = (
  callback: (recording: boolean) => void,
) => listen<boolean>("recording-state-changed", (e) => callback(e.payload));

export const onTranscriptionStarted = (callback: () => void) =>
  listen("transcription-started", () => callback());

export const onTranscriptionComplete = (callback: (text: string) => void) =>
  listen<string>("transcription-complete", (e) => callback(e.payload));

export const onDownloadProgress = (
  callback: (progress: DownloadProgress) => void,
) => listen<DownloadProgress>("download-progress", (e) => callback(e.payload));

export const onDownloadComplete = (callback: (modelId: string) => void) =>
  listen<string>("download-complete", (e) => callback(e.payload));

export const onLiveTranscriptionPartial = (
  callback: (text: string) => void,
) => listen<string>("live-transcription-partial", (e) => callback(e.payload));

export const onError = (callback: (error: string) => void) =>
  listen<string>("error", (e) => callback(e.payload));

export const onMicTestLevel = (callback: (level: number) => void) =>
  listen<number>("mic-test-level", (e) => callback(e.payload));

export const onTtsStateChanged = (callback: (speaking: boolean) => void) =>
  listen<boolean>("tts-state-changed", (e) => callback(e.payload));

export const onTtsText = (callback: (text: string) => void) =>
  listen<string>("tts-text", (e) => callback(e.payload));
