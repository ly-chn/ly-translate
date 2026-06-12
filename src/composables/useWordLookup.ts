import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { WordDefinition } from "../types";

export type TooltipMode = "definition" | "translation" | "loading";

export function useWordLookup() {
  const definition = ref<WordDefinition | null>(null);
  const translationText = ref("");
  const mode = ref<TooltipMode>("loading");
  const tooltipPos = ref({ x: 0, y: 0 });
  const visible = ref(false);

  let hideTimer: ReturnType<typeof setTimeout> | null = null;
  let hoverTimer: ReturnType<typeof setTimeout> | null = null;
  let lookupGen = 0;
  let lastWord = "";

  function showTooltip(x: number, y: number) {
    tooltipPos.value = { x, y };
    visible.value = true;
    hideTimer && clearTimeout(hideTimer);
  }

  function hide(delay = 150) {
    hoverTimer && clearTimeout(hoverTimer);
    hoverTimer = null;
    lookupGen++;
    hideTimer = setTimeout(() => {
      visible.value = false;
      definition.value = null;
      translationText.value = "";
      lastWord = "";
    }, delay);
  }

  // Hover word lookup (with delay)
  function lookup(word: string, lang: string, x: number, y: number, delay = 300) {
    const w = word.trim();
    if (!w || w.length < 2) return;
    if (w === lastWord && visible.value && mode.value === "definition") {
      tooltipPos.value = { x, y };
      return;
    }

    hoverTimer && clearTimeout(hoverTimer);
    hideTimer && clearTimeout(hideTimer);

    hoverTimer = setTimeout(async () => {
      const gen = ++lookupGen;
      lastWord = w;
      showTooltip(x, y);
      mode.value = "loading";
      definition.value = null;

      try {
        const result = await invoke<WordDefinition>("lookup_word", { word: w, lang });
        if (gen !== lookupGen) return;
        definition.value = result;
        mode.value = "definition";
      } catch {
        if (gen !== lookupGen) return;
        definition.value = {
          word: w, phonetic: "",
          definitions: [{ pos: "", meaning: "查询失败" }],
          examples: [],
        };
        mode.value = "definition";
      }
    }, delay);
  }

  // Selection: immediate translate
  async function quickTranslate(text: string, from: string, to: string, x: number, y: number) {
    const t = text.trim();
    if (!t || t.length < 2) return;

    hoverTimer && clearTimeout(hoverTimer);
    hideTimer && clearTimeout(hideTimer);
    lastWord = "";
    const gen = ++lookupGen;

    showTooltip(x, y);
    mode.value = "loading";
    translationText.value = "";

    try {
      const result = await invoke<string>("translate", { text: t, from, to, style: "selection" });
      if (gen !== lookupGen) return;
      translationText.value = result;
      mode.value = "translation";
    } catch {
      if (gen !== lookupGen) return;
      translationText.value = "翻译失败";
      mode.value = "translation";
    }
  }

  function keepVisible() {
    hideTimer && clearTimeout(hideTimer);
  }

  return {
    definition, translationText, mode, tooltipPos, visible,
    lookup, quickTranslate, hide, keepVisible,
  };
}
