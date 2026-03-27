import { invoke } from "@tauri-apps/api/core";
import type { AppConfig, AudioDevice, ModelInfo, PermissionStatus, SystemInfo, TtsModelInfo } from "./types";

export const getConfig = () => invoke<AppConfig>("get_config");

export const saveConfig = (config: AppConfig) =>
  invoke("save_config", { config });

export const isFirstRun = () => invoke<boolean>("is_first_run");

export const getSystemInfo = () => invoke<SystemInfo>("get_system_info");

export const listAudioDevices = () =>
  invoke<AudioDevice[]>("list_audio_devices");

export const listModels = () => invoke<ModelInfo[]>("list_models");

export const downloadModel = (modelId: string) =>
  invoke("download_model", { modelId });

export const deleteModel = (modelId: string) =>
  invoke("delete_model", { modelId });

export const loadModel = (modelId: string) =>
  invoke("load_model", { modelId });

export const updateHotkey = (newHotkey: string): Promise<void> =>
  invoke("update_hotkey", { newHotkey });

export const suspendHotkey = () => invoke("suspend_hotkey");

export const resumeHotkey = () => invoke("resume_hotkey");

export const updateHotkeyPtt = (newHotkey: string): Promise<void> =>
  invoke("update_hotkey_ptt", { newHotkey });

export const setAutoPaste = (enabled: boolean) =>
  invoke("set_auto_paste", { enabled });

export const setVerbatimMode = (enabled: boolean) =>
  invoke("set_verbatim_mode", { enabled });

export const setLiveMode = (enabled: boolean) =>
  invoke("set_live_mode", { enabled });

export const setLanguage = (language: string) =>
  invoke("set_language", { language });

export const setUiLocale = (locale: string) =>
  invoke("set_ui_locale", { locale });

export const setAudioDevice = (deviceName: string | null) =>
  invoke("set_audio_device", { deviceName });

export const testMicrophone = () => invoke("test_microphone");

export const markSetupComplete = () => invoke("mark_setup_complete");

export const getRecordingState = () =>
  invoke<boolean>("get_recording_state");

export const checkPermissions = () =>
  invoke<PermissionStatus>("check_permissions");

export const requestMicrophonePermission = () =>
  invoke("request_microphone_permission");

export const openAccessibilityPreferences = () =>
  invoke("open_accessibility_preferences");

// TTS commands

export const setTtsEnabled = (enabled: boolean) =>
  invoke("set_tts_enabled", { enabled });

export const setTtsModel = (modelId: string) =>
  invoke("set_tts_model", { modelId });

export const setTtsRate = (rate: number) =>
  invoke("set_tts_rate", { rate });

export const updateTtsHotkey = (newHotkey: string): Promise<void> =>
  invoke("update_tts_hotkey", { newHotkey });

export const speakText = (text: string) =>
  invoke("speak_text", { text });

export const stopSpeaking = () => invoke("stop_speaking");

export const listTtsModels = () =>
  invoke<TtsModelInfo[]>("list_tts_models");

export const downloadTtsVoice = (voiceId: string) =>
  invoke("download_tts_voice", { voiceId });

export const downloadPiper = () => invoke("download_piper");

export const isPiperInstalled = () => invoke<boolean>("is_piper_installed");

export const deleteTtsVoice = (voiceId: string) =>
  invoke("delete_tts_voice", { voiceId });

export const checkIsSpeaking = () => invoke<boolean>("is_speaking");
