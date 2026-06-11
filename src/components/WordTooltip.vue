<script setup lang="ts">
import { computed } from "vue";
import type { WordDefinition } from "../types";
import type { TooltipMode } from "../composables/useWordLookup";

const props = defineProps<{
  definition: WordDefinition | null;
  translation: string;
  mode: TooltipMode;
  pos: { x: number; y: number };
  visible: boolean;
}>();

defineEmits(["mouseenter", "mouseleave"]);

const style = computed(() => ({
  left: Math.min(props.pos.x + 14, window.innerWidth - 360) + "px",
  top: Math.min(props.pos.y + 18, window.innerHeight - 260) + "px",
}));
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
      <!-- Loading -->
      <template v-if="mode === 'loading'">
        <div class="tooltip-word pulse">···</div>
      </template>

      <!-- Word definition -->
      <template v-else-if="mode === 'definition' && definition">
        <div class="tooltip-word">{{ definition.word }}</div>
        <div v-if="definition.phonetic" class="tooltip-phonetic">/{{ definition.phonetic }}/</div>
        <div v-for="(def, i) in definition.definitions" :key="i" class="tooltip-def">
          <span v-if="def.pos" class="tooltip-pos">{{ def.pos }}</span>
          {{ def.meaning }}
        </div>
        <div v-for="(ex, i) in definition.examples" :key="'e'+i" class="tooltip-example">
          {{ ex }}
        </div>
      </template>

      <!-- Translation result -->
      <template v-else-if="mode === 'translation' && translation">
        <div class="tooltip-translation">{{ translation }}</div>
      </template>
    </div>
  </Teleport>
</template>

<style scoped>
.tooltip-translation {
  font-size: 13px;
  line-height: 1.5;
  color: var(--text);
  max-width: 320px;
  white-space: pre-wrap;
  word-wrap: break-word;
}
</style>
