import { useState, useEffect, useRef } from "react";
import { suspendHotkey, resumeHotkey } from "../lib/commands";
import { useI18n } from "../lib/i18n";

interface Props {
  label: string;
  currentHotkey: string;
  onSave: (hotkey: string) => Promise<void>;
  onUpdate: () => void;
  allowClear?: boolean;
}

function codeToKey(code: string): string | null {
  if (code.startsWith("Key")) return code.slice(3);
  if (code.startsWith("Digit")) return code.slice(5);
  if (code.startsWith("Numpad")) return "num" + code.slice(6);
  if (/^F\d+$/.test(code)) return code;

  const map: Record<string, string> = {
    Space: "Space",
    Enter: "Return",
    Backspace: "Backspace",
    Delete: "Delete",
    Insert: "Insert",
    Help: "Insert",
    Tab: "Tab",
    ArrowUp: "Up",
    ArrowDown: "Down",
    ArrowLeft: "Left",
    ArrowRight: "Right",
    Home: "Home",
    End: "End",
    PageUp: "PageUp",
    PageDown: "PageDown",
    Minus: "-",
    Equal: "=",
    BracketLeft: "[",
    BracketRight: "]",
    Semicolon: ";",
    Quote: "'",
    Backquote: "`",
    Backslash: "\\",
    Comma: ",",
    Period: ".",
    Slash: "/",
  };

  return map[code] || null;
}

export function HotkeyPicker({ label, currentHotkey, onSave, onUpdate, allowClear }: Props) {
  const { t } = useI18n();
  const [capturing, setCapturing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [pressed, setPressed] = useState<string | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const formatHotkeyDisplay = (hotkey: string): string => {
    if (!hotkey) return t("hotkey.notSet");
    return hotkey
      .replace("CmdOrCtrl", "\u2318")
      .replace("Shift", "\u21e7")
      .replace("Alt", "\u2325")
      .replace("Space", t("hotkey.space"))
      .replace("Return", "\u21b5")
      .replace(/\+/g, " + ");
  };

  useEffect(() => {
    if (capturing) {
      suspendHotkey().catch(() => {});
      setPressed(null);
      inputRef.current?.focus();
    } else {
      resumeHotkey().catch(() => {});
      setPressed(null);
    }
  }, [capturing]);

  const handleKeyDown = async (e: React.KeyboardEvent) => {
    if (!capturing) return;
    e.preventDefault();
    e.stopPropagation();

    if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) {
      const preview: string[] = [];
      if (e.metaKey || e.ctrlKey) preview.push("\u2318");
      if (e.shiftKey) preview.push("\u21e7");
      if (e.altKey) preview.push("\u2325");
      setPressed(preview.join(" + ") + " + ...");
      return;
    }

    if (e.code === "Escape") {
      setCapturing(false);
      return;
    }

    if (allowClear && (e.code === "Backspace" || e.code === "Delete") && !e.metaKey && !e.ctrlKey && !e.shiftKey && !e.altKey) {
      setCapturing(false);
      try {
        await onSave("");
        setError(null);
        onUpdate();
      } catch (err) {
        setError(String(err));
      }
      return;
    }

    let key = codeToKey(e.code);
    if (!key) {
      const keyFallback: Record<string, string> = {
        Insert: "Insert",
        Help: "Insert",
      };
      key = keyFallback[e.key] || null;
    }
    if (!key) return;

    const parts: string[] = [];
    if (e.metaKey || e.ctrlKey) parts.push("CmdOrCtrl");
    if (e.shiftKey) parts.push("Shift");
    if (e.altKey) parts.push("Alt");

    parts.push(key);
    const hotkey = parts.join("+");
    setCapturing(false);

    try {
      await onSave(hotkey);
      setError(null);
      onUpdate();
    } catch (err) {
      setError(String(err));
    }
  };

  return (
    <div className="hotkey-picker">
      <label>{label}</label>
      <div className="hotkey-display">
        {capturing ? (
          <input
            ref={inputRef}
            className="hotkey-input capturing"
            onKeyDown={handleKeyDown}
            onBlur={() => setCapturing(false)}
            autoFocus
            readOnly
            value={pressed || ""}
            placeholder={allowClear ? t("hotkey.placeholderClear") : t("hotkey.placeholder")}
          />
        ) : (
          <div className="hotkey-value" onClick={() => setCapturing(true)}>
            <span className="hotkey-keys">
              {formatHotkeyDisplay(currentHotkey)}
            </span>
            <button
              className="btn btn-sm btn-secondary"
              onClick={(e) => {
                e.stopPropagation();
                setCapturing(true);
              }}
            >
              {t("hotkey.change")}
            </button>
          </div>
        )}
      </div>
      {error && <div className="error-text">{error}</div>}
    </div>
  );
}
