import { createContext, useContext } from "react";

export type UILocale = "en" | "fr";

const translations = {
  // App
  "app.loading": { en: "Loading...", fr: "Chargement..." },

  // Settings
  "settings.recording": { en: "Recording...", fr: "Enregistrement..." },
  "settings.transcribing": { en: "Transcribing...", fr: "Transcription..." },
  "settings.ready": { en: "Ready", fr: "Pret" },
  "settings.lastTranscription": {
    en: "Last transcription:",
    fr: "Derniere transcription :",
  },
  "settings.manageModels": {
    en: "Manage models",
    fr: "Gerer les modeles",
  },
  "settings.back": { en: "Back", fr: "Retour" },
  "settings.shortcuts": { en: "Shortcuts", fr: "Raccourcis" },
  "settings.hotkeyToggle": {
    en: "Toggle (press to start/stop)",
    fr: "Toggle (appuyer pour demarrer/arreter)",
  },
  "settings.hotkeyPtt": {
    en: "Push to talk",
    fr: "Maintenir pour parler",
  },
  "settings.pttHelp": {
    en: "Hold the shortcut to record, release to transcribe. Delete to clear.",
    fr: "Maintenez le raccourci pour enregistrer, relachez pour transcrire. Suppr pour effacer.",
  },
  "settings.general": { en: "General", fr: "General" },
  "settings.autoPaste": {
    en: "Auto-paste",
    fr: "Coller automatiquement",
  },
  "settings.autoPasteHelp": {
    en: "Automatically pastes transcribed text at cursor position",
    fr: "Colle automatiquement le texte transcrit a la position du curseur",
  },
  "settings.verbatimMode": {
    en: "Full verbatim transcription",
    fr: "Transcription integrale mot a mot",
  },
  "settings.verbatimModeHelp": {
    en: "Transcribes every word exactly as spoken, without summarizing or rephrasing",
    fr: "Transcrit chaque mot exactement comme prononce, sans resumer ni reformuler",
  },
  "settings.liveMode": {
    en: "Live note-taking",
    fr: "Prise de notes en direct",
  },
  "settings.liveModeHelp": {
    en: "Transcribes in real-time while you speak, showing text as it is recognized",
    fr: "Transcrit en temps reel pendant que vous parlez, affichant le texte au fur et a mesure",
  },
  "settings.liveTranscription": {
    en: "Live transcription",
    fr: "Transcription en direct",
  },
  "settings.liveWaiting": {
    en: "Listening...",
    fr: "Ecoute en cours...",
  },
  "settings.fileUpload": {
    en: "Transcribe a file",
    fr: "Transcrire un fichier",
  },
  "settings.selectFile": {
    en: "Choose an audio file",
    fr: "Choisir un fichier audio",
  },
  "settings.transcribeFile": {
    en: "Transcribe",
    fr: "Transcrire",
  },
  "settings.fileUploadHelp": {
    en: "Supported formats: WAV, MP3, FLAC, OGG",
    fr: "Formats supportes : WAV, MP3, FLAC, OGG",
  },
  "settings.noFileSelected": {
    en: "No file selected",
    fr: "Aucun fichier selectionne",
  },
  "settings.transcription": { en: "Transcription", fr: "Transcription" },
  "settings.audio": { en: "Audio", fr: "Audio" },
  "settings.interface": { en: "Interface", fr: "Interface" },
  "settings.uiLanguage": {
    en: "Interface language",
    fr: "Langue de l'interface",
  },

  // Setup wizard
  "wizard.welcome": {
    en: "Welcome to LocalWhisper",
    fr: "Bienvenue dans LocalWhisper",
  },
  "wizard.subtitle": {
    en: "Local and private voice transcription",
    fr: "Transcription vocale locale et privee",
  },
  "wizard.yourMachine": { en: "Your machine", fr: "Votre machine" },
  "wizard.ramCores": {
    en: "{ram} GB RAM - {cores} CPU cores - {os}",
    fr: "{ram} Go RAM - {cores} coeurs CPU - {os}",
  },
  "wizard.chooseModel": {
    en: "Choose a model",
    fr: "Choisissez un modele",
  },
  "wizard.recommended": { en: "Recommended", fr: "Recommande" },
  "wizard.installContinue": {
    en: "Install and continue",
    fr: "Installer et continuer",
  },
  "wizard.downloading": {
    en: "Downloading...",
    fr: "Telechargement en cours...",
  },
  "wizard.downloadHelp": {
    en: "This may take a few minutes depending on your connection.",
    fr: "Cela peut prendre quelques minutes selon votre connexion.",
  },
  "wizard.ready": { en: "Ready!", fr: "Pret !" },
  "wizard.readyMessage": {
    en: "LocalWhisper is configured. Use the keyboard shortcut to start dictating.",
    fr: "LocalWhisper est configure. Utilisez le raccourci clavier pour commencer a dicter.",
  },

  // Permissions
  "permissions.checking": {
    en: "Checking permissions...",
    fr: "Verification des permissions...",
  },
  "permissions.required": {
    en: "Permissions required",
    fr: "Permissions requises",
  },
  "permissions.description": {
    en: "LocalWhisper needs these permissions to work properly.",
    fr: "LocalWhisper a besoin de ces permissions pour fonctionner correctement.",
  },
  "permissions.microphone": { en: "Microphone", fr: "Microphone" },
  "permissions.microphoneDesc": {
    en: "To capture your voice and transcribe it to text.",
    fr: "Pour capturer votre voix et la transcrire en texte.",
  },
  "permissions.granted": { en: "Granted", fr: "Accorde" },
  "permissions.allow": { en: "Allow", fr: "Autoriser" },
  "permissions.accessibility": {
    en: "Accessibility",
    fr: "Accessibilite",
  },
  "permissions.accessibilityDesc": {
    en: "To automatically paste transcribed text (Cmd+V simulation).",
    fr: "Pour coller automatiquement le texte transcrit (simulation Cmd+V).",
  },
  "permissions.openSettings": {
    en: "Open settings",
    fr: "Ouvrir les reglages",
  },
  "permissions.accessibilityHelp": {
    en: "In System Settings, enable LocalWhisper in the Accessibility list, then come back here. This page updates automatically.",
    fr: "Dans les Reglages Systeme, activez LocalWhisper dans la liste Accessibilite, puis revenez ici. La page se met a jour automatiquement.",
  },
  "permissions.skipButton": {
    en: "Continue without checking",
    fr: "Continuer sans verifier",
  },
  "permissions.skipHelp": {
    en: "The microphone will work if permission was granted in System Settings. Accessibility is optional.",
    fr: "Le micro fonctionnera si la permission a ete accordee dans les Reglages Systeme. L'accessibilite est optionnelle.",
  },

  // Model catalog
  "catalog.systemInfo": {
    en: "{os} ({arch}) - {ram} GB RAM - {cores} cores",
    fr: "{os} ({arch}) - {ram} Go RAM - {cores} coeurs",
  },
  "catalog.installed": { en: "Installed models", fr: "Modeles installes" },
  "catalog.available": {
    en: "Available models",
    fr: "Modeles disponibles",
  },

  // Model selector
  "modelSelector.label": { en: "Active model", fr: "Modele actif" },
  "modelSelector.placeholder": {
    en: "Select a model",
    fr: "Selectionnez un modele",
  },
  "modelSelector.noModels": {
    en: 'No models installed. Go to "Manage models" to download one.',
    fr: 'Aucun modele installe. Allez dans "Gerer les modeles" pour en telecharger un.',
  },

  // Model card
  "modelCard.delete": { en: "Delete", fr: "Supprimer" },
  "modelCard.install": { en: "Install", fr: "Installer" },

  // Hotkey picker
  "hotkey.notSet": { en: "Not set", fr: "Non defini" },
  "hotkey.space": { en: "Space", fr: "Espace" },
  "hotkey.placeholderClear": {
    en: "Shortcut... (Del to clear)",
    fr: "Raccourci... (Suppr pour effacer)",
  },
  "hotkey.placeholder": {
    en: "Press your shortcut...",
    fr: "Appuyez sur votre raccourci...",
  },
  "hotkey.change": { en: "Change", fr: "Modifier" },

  // Language selector (transcription language)
  "langSelector.label": {
    en: "Transcription language",
    fr: "Langue de transcription",
  },
  "langSelector.auto": { en: "Automatic", fr: "Automatique" },
  "langSelector.fr": { en: "French", fr: "Francais" },
  "langSelector.en": { en: "English", fr: "Anglais" },
  "langSelector.es": { en: "Spanish", fr: "Espagnol" },
  "langSelector.de": { en: "German", fr: "Allemand" },
  "langSelector.it": { en: "Italian", fr: "Italien" },
  "langSelector.pt": { en: "Portuguese", fr: "Portugais" },
  "langSelector.nl": { en: "Dutch", fr: "Neerlandais" },
  "langSelector.ja": { en: "Japanese", fr: "Japonais" },
  "langSelector.zh": { en: "Chinese", fr: "Chinois" },
  "langSelector.ko": { en: "Korean", fr: "Coreen" },
  "langSelector.ru": { en: "Russian", fr: "Russe" },
  "langSelector.ar": { en: "Arabic", fr: "Arabe" },
  "langSelector.pl": { en: "Polish", fr: "Polonais" },
  "langSelector.uk": { en: "Ukrainian", fr: "Ukrainien" },
  "langSelector.tr": { en: "Turkish", fr: "Turc" },

  // Audio device selector
  "audio.microphone": { en: "Microphone", fr: "Microphone" },
  "audio.default": { en: "Default", fr: "Par defaut" },
  "audio.defaultSuffix": { en: "(default)", fr: "(defaut)" },
  "audio.testing": { en: "Testing...", fr: "Test en cours..." },
  "audio.testMic": { en: "Test microphone", fr: "Tester le micro" },
  "audio.micOk": { en: "Microphone OK", fr: "Micro OK" },
  "audio.noSound": {
    en: "No sound detected",
    fr: "Aucun son detecte",
  },
  "audio.error": { en: "Error", fr: "Erreur" },

  // Update checker
  "update.title": { en: "Updates", fr: "Mises a jour" },
  "update.checkButton": { en: "Check for updates", fr: "Verifier" },
  "update.checking": { en: "Checking...", fr: "Verification..." },
  "update.upToDate": { en: "Up to date", fr: "A jour" },
  "update.installButton": {
    en: "Install and restart",
    fr: "Installer et redemarrer",
  },
  "update.releaseNotes": { en: "What's new:", fr: "Nouveautes :" },
  "update.installing": { en: "Installing...", fr: "Installation..." },

  // Download progress
  "download.unit": { en: "MB", fr: "Mo" },

  // TTS (Text-to-Speech)
  "settings.tts": { en: "Text-to-Speech", fr: "Lecture vocale" },
  "settings.ttsEnabled": {
    en: "Enable text-to-speech",
    fr: "Activer la lecture vocale",
  },
  "settings.ttsEnabledHelp": {
    en: "Select text anywhere, press the shortcut to read it aloud",
    fr: "Selectionnez du texte n'importe ou, appuyez sur le raccourci pour le lire a voix haute",
  },
  "settings.ttsRate": { en: "Speed", fr: "Vitesse" },
  "settings.ttsRateSlow": { en: "Slow", fr: "Lent" },
  "settings.ttsRateFast": { en: "Fast", fr: "Rapide" },
  "settings.ttsHotkey": {
    en: "Read selection",
    fr: "Lire la selection",
  },
  "settings.ttsHotkeyHelp": {
    en: "Press to read selected text, press again to stop. Delete to clear.",
    fr: "Appuyez pour lire le texte selectionne, reappuyez pour arreter. Suppr pour effacer.",
  },
  "settings.ttsSpeaking": { en: "Reading...", fr: "Lecture..." },
  "settings.ttsWordsPerMin": {
    en: "{rate} words/min",
    fr: "{rate} mots/min",
  },
  "settings.ttsReadingText": {
    en: "Reading:",
    fr: "Lecture :",
  },
  "settings.ttsVoiceModel": { en: "Voice", fr: "Voix" },
  "settings.ttsNoVoice": {
    en: "No voice installed. Download one below.",
    fr: "Aucune voix installee. Telechargez-en une ci-dessous.",
  },
  "settings.ttsInstallPiper": {
    en: "Install Piper TTS engine",
    fr: "Installer le moteur Piper TTS",
  },
  "settings.ttsInstallPiperHelp": {
    en: "Piper is a fast neural text-to-speech engine. Required for realistic voices.",
    fr: "Piper est un moteur de synthese vocale neurale rapide. Requis pour les voix realistes.",
  },
  "settings.ttsPiperInstalled": {
    en: "Piper engine installed",
    fr: "Moteur Piper installe",
  },
  "settings.ttsAvailableVoices": {
    en: "Available voices",
    fr: "Voix disponibles",
  },
  "settings.ttsInstall": { en: "Install", fr: "Installer" },
  "settings.ttsDelete": { en: "Delete", fr: "Supprimer" },
  "settings.ttsActive": { en: "Active", fr: "Active" },
  "settings.ttsUse": { en: "Use", fr: "Utiliser" },
} as const;

export type TranslationKey = keyof typeof translations;

export function t(key: TranslationKey, locale: UILocale, params?: Record<string, string | number>): string {
  const entry = translations[key];
  let text: string = entry[locale] ?? entry.en;
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      text = text.replace(`{${k}}`, String(v));
    }
  }
  return text;
}

export interface I18nContextValue {
  locale: UILocale;
  setLocale: (locale: UILocale) => void;
  t: (key: TranslationKey, params?: Record<string, string | number>) => string;
}

export const I18nContext = createContext<I18nContextValue>({
  locale: "en",
  setLocale: () => {},
  t: (key) => t(key, "en"),
});

export function useI18n() {
  return useContext(I18nContext);
}
