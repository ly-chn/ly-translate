<script setup lang="ts">
import { computed } from "vue";
import type { WordDefinition } from "../types";
import type { TooltipMode } from "../composables/useWordLookup";

const props = defineProps<{
  definition: WordDefinition | null;
  mode: TooltipMode;
  pos: { x: number; y: number };
  visible: boolean;
}>();

defineEmits(["mouseenter", "mouseleave"]);

const style = computed(() => ({
  left: Math.min(props.pos.x + 14, window.innerWidth - 420) + "px",
  top: Math.min(props.pos.y + 18, window.innerHeight - 360) + "px",
}));

let audio: HTMLAudioElement | null = null;

function play(url?: string) {
  if (!url) return;
  try {
    audio?.pause();
    audio = new Audio(url);
    audio.play().catch(() => {});
  } catch {
    /* ignore */
  }
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="visible"
      class="word-tooltip"
      :style="style"
      @mouseenter="$emit('mouseenter')"
      @mouseleave="$emit('mouseleave')"
    >
      <template v-if="mode === 'loading'">
        <div class="tooltip-word pulse">···</div>
      </template>

      <template v-else-if="mode === 'definition' && definition">
        <div class="tooltip-head">
          <div class="tooltip-word">{{ definition.word }}</div>
          <div class="speech-btns" v-if="definition.ukSpeech || definition.usSpeech">
            <button
              v-if="definition.ukSpeech"
              class="speech-btn"
              title="英音"
              @click.stop="play(definition.ukSpeech)"
            >英</button>
            <button
              v-if="definition.usSpeech"
              class="speech-btn"
              title="美音"
              @click.stop="play(definition.usSpeech)"
            >美</button>
          </div>
        </div>

        <div v-if="definition.ukPhonetic || definition.usPhonetic" class="tooltip-phonetic">
          <span v-if="definition.ukPhonetic">英 /{{ definition.ukPhonetic }}/</span>
          <span v-if="definition.usPhonetic" class="ph-gap">美 /{{ definition.usPhonetic }}/</span>
        </div>
        <div v-else-if="definition.phonetic" class="tooltip-phonetic">/{{ definition.phonetic }}/</div>

        <div class="tooltip-defs">
          <div v-for="(def, i) in definition.definitions" :key="i" class="tooltip-def">
            <span v-if="def.pos" class="tooltip-pos">{{ def.pos }}</span>
            <span class="tooltip-meaning">{{ def.meaning }}</span>
          </div>
        </div>

        <div v-if="definition.examples?.length" class="tooltip-examples">
          <div v-for="(ex, i) in definition.examples" :key="'e'+i" class="tooltip-example">
            {{ ex }}
          </div>
        </div>
      </template>
    </div>
  </Teleport>
</template>

<style scoped>
.tooltip-head {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 2px;
}
.tooltip-word {
  margin-bottom: 0;
}
.speech-btns {
  display: flex;
  gap: 4px;
  margin-left: auto;
}
.speech-btn {
  width: 22px;
  height: 22px;
  border-radius: 5px;
  font-size: 11px;
  font-weight: 600;
  color: var(--accent-text);
  background: var(--accent-glow);
  border: 1px solid rgba(110, 142, 251, 0.22);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  line-height: 1;
}
.speech-btn:hover {
  color: #fff;
  background: var(--accent);
  border-color: transparent;
}
.ph-gap { margin-left: 10px; }
.tooltip-defs {
  max-height: 200px;
  overflow-y: auto;
  margin-top: 4px;
}
.tooltip-meaning {
  color: var(--text);
}
.tooltip-examples {
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--border-light);
  max-height: 120px;
  overflow-y: auto;
}
</style>
