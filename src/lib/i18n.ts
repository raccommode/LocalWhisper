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
