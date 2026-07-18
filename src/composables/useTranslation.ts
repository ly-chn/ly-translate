import { ref, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

type TranslateChunk = {
  id: number;
  delta: string;
  done: boolean;
};

export function useTranslation() {
  const sourceText = ref("");
  const translatedText = ref("");
  const isTranslating = ref(false);

  let requestId = 0;
  let unlisten: UnlistenFn | null = null;
  let listenReady: Promise<void> | null = null;

  function ensureListen() {
    if (listenReady) return listenReady;
    listenReady = listen<TranslateChunk>("translate-chunk", (e) => {
      const { id, delta, done } = e.payload;
      if (id === 0 || id !== requestId) return;
      if (done) return;
      if (delta) translatedText.value += delta;
    }).then((fn) => {
      unlisten = fn;
    });
    return listenReady;
  }

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

    await ensureListen();

    const id = ++requestId;
    isTranslating.value = true;
    translatedText.value = "";
    try {
      const result = await invoke<string>("translate", {
        text,
        from,
        to,
        style,
        id,
      });
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

  onUnmounted(() => {
    unlisten?.();
    unlisten = null;
    listenReady = null;
  });

  return { sourceText, translatedText, isTranslating, translate, clear };
}
