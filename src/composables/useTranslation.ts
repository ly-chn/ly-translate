import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export function useTranslation() {
  const sourceText = ref("");
  const translatedText = ref("");
  const isTranslating = ref(false);

  let requestId = 0;

  async function translate(
    text: string,
    from: string,
    to: string,
    style: string
  ) {
    if (!text.trim()) {
      translatedText.value = "";
      return;
    }

    const id = ++requestId;
    isTranslating.value = true;
    try {
      const result = await invoke<string>("translate", { text, from, to, style });
      if (id !== requestId) return;
      translatedText.value = result;
    } catch (e: unknown) {
      if (id !== requestId) return;
      if (e === "cancelled") return;
      translatedText.value = `翻译失败: ${e}`;
    } finally {
      if (id === requestId) {
        isTranslating.value = false;
      }
    }
  }

  function clear() {
    requestId++;
    sourceText.value = "";
    translatedText.value = "";
    isTranslating.value = false;
  }

  return { sourceText, translatedText, isTranslating, translate, clear };
}
