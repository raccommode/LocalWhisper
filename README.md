<div align="center">

# LocalWhisper

**Local, private speech-to-text transcription powered by Whisper AI.**

[![Release](https://img.shields.io/github/v/release/raccommode/LocalWhisper?style=flat-square)](https://github.com/raccommode/LocalWhisper/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![macOS](https://img.shields.io/badge/macOS-supported-success?style=flat-square&logo=apple)](#download)
[![Windows](https://img.shields.io/badge/Windows-supported-success?style=flat-square&logo=windows)](#download)
[![Linux](https://img.shields.io/badge/Linux-supported-success?style=flat-square&logo=linux)](#download)

Press a hotkey. Speak. Text appears at your cursor.
No cloud, no API keys, no data leaves your machine.

</div>

---

## Features

| | |
|---|---|
| **100% local & private** | All processing stays on your machine — zero network calls |
| **Global hotkey** | Toggle recording or push-to-talk from any app |
| **Auto-paste** | Transcribed text is pasted directly at your cursor |
| **15+ languages** | English, French, Spanish, German, Japanese, Chinese, and more |
| **Multiple models** | From tiny (75 MB) to large (3 GB) — pick the best fit for your hardware |
| **GPU-accelerated** | Metal on macOS, Vulkan on Windows/Linux |
| **System tray** | Runs silently in the background with animated status icons |
| **Auto-updates** | Built-in updater keeps you on the latest version |
| **Bilingual UI** | English and French interface |

## Download

> Download the latest version from the [**Releases page**](https://github.com/raccommode/LocalWhisper/releases/latest).

| Platform | File | Architecture |
|---|---|---|
| **macOS** | `.dmg` | Apple Silicon (M1+) / Intel |
| **Windows** | `.exe` | x64 |
| **Linux** | `.deb` / `.AppImage` | x64 |

Launch the app and the setup wizard will guide you through model download, microphone permissions, and hotkey configuration.

## How it works

```
1. Press the global hotkey        (default: Cmd+Shift+H / Win+Shift+H)
2. Speak
3. Press the hotkey again          (or release key in push-to-talk mode)
4. Text is pasted at your cursor
```

## Build from source

**Prerequisites:** [Node.js](https://nodejs.org/), [pnpm](https://pnpm.io/), [Rust](https://rustup.rs/)

```bash
git clone https://github.com/raccommode/LocalWhisper.git
cd LocalWhisper
pnpm install
pnpm tauri dev      # development
pnpm tauri build    # production build
```

## Tech stack

- **Backend** — Rust, [Tauri 2](https://tauri.app/), [whisper-rs](https://github.com/tazz4843/whisper-rs), cpal, rodio, enigo
- **Frontend** — React 18, TypeScript, Vite

## License

[MIT](LICENSE)
