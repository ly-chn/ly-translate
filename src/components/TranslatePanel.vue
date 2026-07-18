<script setup lang="ts">
import { watch, onUnmounted } from "vue";
import { useTranslation } from "../composables/useTranslation";
import { useWordLookup } from "../composables/useWordLookup";
import WordTooltip from "./WordTooltip.vue";

const props = defineProps<{
  sourceLang: string;
  targetLang: string;
  style: string;
}>();

const {
  sourceText,
  translatedText,
  isTranslating,
  translate,
  clear,
} = useTranslation();

const wordLookup = useWordLookup();
let debounceTimer: ReturnType<typeof setTimeout> | null = null;
let composing = false;

function doTranslate() {
  debounceTimer && clearTimeout(debounceTimer);
  debounceTimer = setTimeout(() => {
    if (sourceText.value.trim()) {
      translate(sourceText.value, props.sourceLang, props.targetLang, props.style);
    } else {
      translatedText.value = "";
    }
  }, 500);
}

function clearAll() {
  debounceTimer && clearTimeout(debounceTimer);
  debounceTimer = null;
  wordLookup.hide();
  clear();
}

function onCompositionStart() { composing = true; }
function onCompositionEnd() { composing = false; doTranslate(); }
function onInput() { if (!composing) doTranslate(); }

watch(
  () => [props.sourceLang, props.targetLang, props.style],
  () => {
    if (sourceText.value.trim()) {
      translate(sourceText.value, props.sourceLang, props.targetLang, props.style);
    }
  }
);

function onSourceSelect(e: MouseEvent) {
  const sel = window.getSelection()?.toString().trim();
  if (sel) wordLookup.lookupSelection(sel, e.clientX, e.clientY);
}

function onOutputMouseUp(e: MouseEvent) {
  const sel = window.getSelection()?.toString().trim();
  if (sel) wordLookup.lookupSelection(sel, e.clientX, e.clientY);
}

onUnmounted(() => { debounceTimer && clearTimeout(debounceTimer); });
</script>

<template>
  <div class="main">
    <!-- Left: Source -->
    <div class="pane">
      <div class="pane-head">
        <span class="pane-label">原文</span>
        <span class="char-count" v-if="sourceText">{{ sourceText.length }}</span>
        <button
          v-if="sourceText || translatedText"
          class="clear-btn"
          title="清空"
          @click="clearAll"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none">
            <path d="M18 6L6 18M6 6l12 12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
          </svg>
          清空
        </button>
      </div>
      <textarea
        v-model="sourceText"
        @input="onInput"
        @compositionstart="onCompositionStart"
        @compositionend="onCompositionEnd"
        @mouseup="onSourceSelect"
        placeholder="输入或粘贴文本..."
        spellcheck="false"
      ></textarea>
    </div>

    <div class="divider"><div class="divider-line"></div></div>

    <!-- Right: Target -->
    <div class="pane">
      <div class="pane-head">
        <span class="pane-label">译文</span>
        <span v-if="isTranslating" class="status-dot pulse"></span>
      </div>
      <div
        class="output"
        @mouseup="onOutputMouseUp"
      >
        <pre v-if="translatedText">{{ translatedText }}</pre>
        <div v-else class="empty-hint">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" opacity="0.15">
            <path d="M3 5h12M9 3v2m1.048 3.5A18.024 18.024 0 003.186 13m2.87-5.5a18.02 18.02 0 005.89 8.243M12 21l3.75-7.5L21 21m-3-12h.01" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </div>
      </div>
    </div>
  </div>

  <WordTooltip
    :definition="wordLookup.definition.value"
    :mode="wordLookup.mode.value"
    :pos="wordLookup.tooltipPos.value"
    :visible="wordLookup.visible.value"
    @mouseenter="wordLookup.keepVisible()"
    @mouseleave="wordLookup.hide()"
  />
</template>

<style scoped>
.main { flex: 1; display: flex; overflow: hidden; }

.pane { flex: 1; display: flex; flex-direction: column; min-width: 0; }

.pane-head {
  display: flex; align-items: center; gap: 6px;
  padding: 6px 12px 6px 20px; height: 32px; flex-shrink: 0;
  user-select: none; -webkit-user-select: none;
}
.pane-label {
  font-size: 11px; font-weight: 600; color: var(--text-3);
  text-transform: uppercase; letter-spacing: 0.06em;
}
.char-count {
  font-size: 10px; color: var(--text-3); opacity: 0.6;
  font-family: var(--mono);
}
.clear-btn {
  margin-left: auto;
  display: flex; align-items: center; gap: 4px;
  height: 24px; padding: 0 10px; border-radius: 6px;
  font-size: 12px; font-weight: 500;
  color: var(--accent-text);
  background: var(--accent-glow);
  border: 1px solid rgba(110, 142, 251, 0.22);
  transition: all var(--transition);
}
.clear-btn:hover {
  color: #fff;
  background: var(--accent);
  border-color: transparent;
}
.status-dot {
  width: 5px; height: 5px; border-radius: 50%;
  background: var(--accent);
}

textarea {
  flex: 1; resize: none; border: none; background: transparent;
  padding: 0 20px 20px; line-height: 1.7; font-size: 14px;
  color: var(--text); outline: none; letter-spacing: -0.01em;
}
textarea::placeholder { color: var(--text-3); }

.output {
  flex: 1; overflow-y: auto; padding: 0 20px 20px;
  cursor: text; user-select: text;
}
.output pre {
  font-family: inherit; white-space: pre-wrap; word-wrap: break-word;
  line-height: 1.7; font-size: 14px; margin: 0; letter-spacing: -0.01em;
}
.empty-hint {
  display: flex; align-items: center; justify-content: center;
  height: 100%; color: var(--text-3);
}

.divider { width: 1px; display: flex; align-items: stretch; flex-shrink: 0; }
.divider-line {
  width: 1px;
  background: linear-gradient(180deg, transparent 0%, var(--border) 20%, var(--border) 80%, transparent 100%);
}
</style>
