import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { WordDefinition } from "../types";

export type TooltipMode = "definition" | "loading";

export function useWordLookup() {
  const definition = ref<WordDefinition | null>(null);
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
      lastWord = "";
    }, delay);
  }

  function detectLang(text: string): string {
    const zh = (text.match(/[\u4e00-\u9fff]/g) || []).length;
    return zh >= Math.max(1, text.length / 4) ? "zh" : "en";
  }

  function worthLookup(text: string): boolean {
    const t = text.trim();
    if (!t) return false;
    // 词典适合词/短语，过长当句子跳过
    if (t.length > 40) return false;
    if (/[\u4e00-\u9fff]/.test(t)) return t.length >= 1;
    return t.length >= 2;
  }

  async function doLookup(word: string, lang: string, x: number, y: number) {
    const w = word.trim();
    if (!worthLookup(w)) return;

    hoverTimer && clearTimeout(hoverTimer);
    hideTimer && clearTimeout(hideTimer);

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
        word: w,
        phonetic: "",
        definitions: [{ pos: "", meaning: "未找到词条" }],
        examples: [],
      };
      mode.value = "definition";
    }
  }

  /** 悬停查词（带延迟） */
  function lookup(word: string, lang: string, x: number, y: number, delay = 300) {
    const w = word.trim();
    if (!worthLookup(w)) return;
    if (w === lastWord && visible.value && mode.value === "definition") {
      tooltipPos.value = { x, y };
      return;
    }

    hoverTimer && clearTimeout(hoverTimer);
    hideTimer && clearTimeout(hideTimer);

    hoverTimer = setTimeout(() => {
      doLookup(w, lang || detectLang(w), x, y);
    }, delay);
  }

  /** 选中查词（立即） */
  function lookupSelection(text: string, x: number, y: number) {
    const t = text.trim();
    if (!worthLookup(t)) return;
    doLookup(t, detectLang(t), x, y);
  }

  function keepVisible() {
    hideTimer && clearTimeout(hideTimer);
  }

  return {
    definition,
    mode,
    tooltipPos,
    visible,
    lookup,
    lookupSelection,
    hide,
    keepVisible,
  };
}
