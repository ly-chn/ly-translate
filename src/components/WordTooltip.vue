<script setup lang="ts">
import { computed, ref } from "vue";
import type { WordDefinition } from "../types";
import type { TooltipMode } from "../composables/useWordLookup";

const props = defineProps<{
  definition: WordDefinition | null;
  mode: TooltipMode;
  pos: { x: number; y: number };
  visible: boolean;
}>();

defineEmits(["mouseenter", "mouseleave"]);

const CARD_W = 300;
const TOOLBAR_H = 52;
const EDGE = 8;

const style = computed(() => {
  const vw = window.innerWidth;
  const vh = window.innerHeight;
  const maxH = Math.min(320, vh - TOOLBAR_H - EDGE * 2);

  let left = props.pos.x + 12;
  left = Math.max(EDGE, Math.min(left, vw - CARD_W - EDGE));

  const preferBelow = props.pos.y + 14;
  const spaceBelow = vh - preferBelow - EDGE;
  let top =
    spaceBelow >= 140
      ? preferBelow
      : props.pos.y - 10 - Math.min(maxH, 220);
  top = Math.max(TOOLBAR_H + EDGE, Math.min(top, vh - 100));

  return {
    left: `${left}px`,
    top: `${top}px`,
    maxHeight: `${maxH}px`,
  };
});

let audio: HTMLAudioElement | null = null;
const playing = ref<"uk" | "us" | null>(null);

function play(kind: "uk" | "us", url?: string) {
  if (!url) return;
  try {
    audio?.pause();
    audio = new Audio(url);
    playing.value = kind;
    audio.onended = () => {
      if (playing.value === kind) playing.value = null;
    };
    audio.onerror = () => {
      playing.value = null;
    };
    audio.play().catch(() => {
      playing.value = null;
    });
  } catch {
    playing.value = null;
  }
}

function splitExample(ex: string): { en: string; zh: string } {
  const lines = ex.split("\n").map((s) => s.trim()).filter(Boolean);
  if (lines.length >= 2) return { en: lines[0], zh: lines.slice(1).join(" ") };
  const m = ex.match(/^(.+?[.!?…])\s*([“"‘'].+)$/);
  if (m) return { en: m[1].trim(), zh: m[2].trim() };
  return { en: ex, zh: "" };
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="visible"
      class="dict-card"
      :style="style"
      @mouseenter="$emit('mouseenter')"
      @mouseleave="$emit('mouseleave')"
    >
      <template v-if="mode === 'loading'">
        <div class="dict-loading">···</div>
      </template>

      <template v-else-if="mode === 'definition' && definition">
        <div class="dict-head">
          <span class="dict-word">{{ definition.word }}</span>
          <span class="dict-ph" v-if="definition.ukPhonetic || definition.usPhonetic || definition.phonetic">
            <button
              v-if="definition.ukSpeech || definition.ukPhonetic"
              class="ph"
              :class="{ on: playing === 'uk' }"
              title="英音"
              @click.stop="play('uk', definition.ukSpeech)"
            >
              英<span v-if="definition.ukPhonetic" class="ipa">/{{ definition.ukPhonetic }}/</span>
            </button>
            <button
              v-if="definition.usSpeech || definition.usPhonetic"
              class="ph"
              :class="{ on: playing === 'us' }"
              title="美音"
              @click.stop="play('us', definition.usSpeech)"
            >
              美<span v-if="definition.usPhonetic" class="ipa">/{{ definition.usPhonetic }}/</span>
            </button>
            <span
              v-if="!definition.ukPhonetic && !definition.usPhonetic && definition.phonetic"
              class="ipa"
            >/{{ definition.phonetic }}/</span>
          </span>
        </div>

        <div v-if="definition.definitions?.length" class="dict-defs">
          <div v-for="(def, i) in definition.definitions" :key="i" class="def">
            <span v-if="def.pos" class="pos">{{ def.pos }}</span>
            <span class="mean">{{ def.meaning }}</span>
          </div>
        </div>

        <div v-if="definition.examples?.length" class="dict-exs">
          <div
            v-for="(ex, i) in definition.examples.slice(0, 2)"
            :key="'e' + i"
            class="ex"
          >
            <template v-for="parts in [splitExample(ex)]" :key="'p' + i">
              <div class="ex-en">{{ parts.en }}</div>
              <div v-if="parts.zh" class="ex-zh">{{ parts.zh }}</div>
            </template>
          </div>
        </div>
      </template>
    </div>
  </Teleport>
</template>

<style scoped>
.dict-card {
  position: fixed;
  z-index: 10000;
  width: 300px;
  max-width: calc(100vw - 20px);
  overflow-y: auto;
  overflow-x: hidden;
  padding: 10px 12px;
  border-radius: 10px;
  background: var(--elevated);
  border: 1px solid var(--border);
  box-shadow: 0 8px 28px rgba(0, 0, 0, 0.22);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  animation: dict-in 0.14s ease;
  font-size: 12.5px;
  color: var(--text);
  line-height: 1.45;
}
:root.light .dict-card {
  box-shadow: 0 8px 28px rgba(0, 0, 0, 0.1);
}
@keyframes dict-in {
  from { opacity: 0; transform: translateY(4px) scale(0.98); }
  to { opacity: 1; transform: none; }
}

.dict-loading {
  color: var(--text-3);
  letter-spacing: 0.2em;
  padding: 4px 0;
  text-align: center;
}

.dict-head {
  display: flex;
  align-items: baseline;
  flex-wrap: wrap;
  gap: 6px 8px;
  margin-bottom: 6px;
}
.dict-word {
  font-size: 16px;
  font-weight: 700;
  letter-spacing: -0.02em;
  color: var(--text);
  line-height: 1.2;
}
.dict-ph {
  display: inline-flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 2px 6px;
  min-width: 0;
}
.ph {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  padding: 0 2px;
  border: none;
  background: transparent;
  color: var(--text-3);
  font-size: 11px;
  font-weight: 500;
  line-height: 1.3;
  border-radius: 3px;
  cursor: pointer;
  transition: color var(--transition);
}
.ph:hover { color: var(--accent-text); }
.ph.on { color: var(--accent); }
.ipa {
  font-family: var(--mono);
  font-size: 10.5px;
  font-weight: 400;
  letter-spacing: -0.02em;
  opacity: 0.85;
  white-space: nowrap;
}

.dict-defs {
  display: flex;
  flex-direction: column;
  gap: 3px;
}
.def {
  display: flex;
  gap: 5px;
  align-items: baseline;
  min-width: 0;
}
.pos {
  flex-shrink: 0;
  font-size: 10px;
  font-weight: 600;
  color: var(--accent-text);
  opacity: 0.85;
  line-height: 1.4;
}
.mean {
  flex: 1;
  min-width: 0;
  font-size: 12.5px;
  color: var(--text);
  line-height: 1.45;
  word-break: break-word;
}

.dict-exs {
  margin-top: 6px;
  padding-top: 6px;
  border-top: 1px solid var(--border-light);
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.ex-en {
  font-size: 11.5px;
  color: var(--text-2);
  line-height: 1.4;
  font-style: italic;
}
.ex-zh {
  font-size: 11px;
  color: var(--text-3);
  line-height: 1.35;
  margin-top: 1px;
}

.dict-card::-webkit-scrollbar { width: 3px; }
.dict-card::-webkit-scrollbar-thumb {
  background: rgba(128, 128, 128, 0.25);
  border-radius: 2px;
}
</style>
