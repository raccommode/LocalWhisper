export interface AppConfig {
  hotkey: string;
  hotkey_ptt: string;
  auto_paste: boolean;
  active_model: string | null;
  language: string;
  audio_device: string | null;
  ui_locale: string;
  first_run_complete: boolean;
}

export interface SystemInfo {
  total_ram_gb: number;
  cpu_cores: number;
  os: string;
  arch: string;
  recommended_model: string;
  recommended_model_reason: string;
}

export interface AudioDevice {
  name: string;
  is_default: boolean;
}

export interface ModelInfo {
  id: string;
  name: string;
  size_bytes: number;
  size_label: string;
  url: string;
  is_english_only: boolean;
  is_quantized: boolean;
  is_downloaded: boolean;
}

export interface DownloadProgress {
  model_id: string;
  downloaded_bytes: number;
  total_bytes: number;
  percent: number;
}

export interface PermissionStatus {
  microphone: boolean;
  accessibility: boolean;
}
