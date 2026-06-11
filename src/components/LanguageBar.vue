<script setup lang="ts">
import { computed } from "vue";
import { useSettingsStore } from "../stores/settings";
import { ALL_LANGUAGES, STYLE_OPTIONS } from "../types";
import type { TranslationStyle } from "../types";

const props = defineProps<{
  sourceLang: string;
  targetLang: string;
  style: TranslationStyle;
}>();

const emit = defineEmits<{
  "update:sourceLang": [v: string];
  "update:targetLang": [v: string];
  "update:style": [v: TranslationStyle];
  openSettings: [];
}>();

const store = useSettingsStore();

const effectiveDark = computed(() => store.settings.darkMode ?? window.matchMedia("(prefers-color-scheme: dark)").matches);

const enabledLanguages = computed(() => {
  const codes = store.settings.enabledLanguages;
  return ALL_LANGUAGES.filter((l) => l.code === "auto" || codes.includes(l.code));
});

function onSourceChange(code: string) {
  emit("update:sourceLang", code);
  if (code === "zh") {
    emit("update:targetLang", "en");
  } else if (code !== "auto") {
    emit("update:targetLang", "zh");
  }
}

function onTargetChange(code: string) {
  emit("update:targetLang", code);
  if (code === "zh") {
    emit("update:sourceLang", "en");
  } else {
    emit("update:sourceLang", "zh");
  }
}

function swap() {
  if (props.sourceLang === "auto") return;
  emit("update:sourceLang", props.targetLang);
  emit("update:targetLang", props.sourceLang);
}
</script>

<template>
  <div class="toolbar">
    <div class="lang-group">
      <select :value="sourceLang" @change="onSourceChange(($event.target as HTMLSelectElement).value)">
        <option v-for="l in enabledLanguages" :key="l.code" :value="l.code">{{ l.name }}</option>
      </select>

      <button class="swap" @click="swap" title="交换语言">
        <svg width="14" height="14" viewBox="0 0 16 16" fill="none">
          <path d="M4 3L1 6L4 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M1 6H11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          <path d="M12 7L15 10L12 13" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          <path d="M15 10H5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </button>

      <select :value="targetLang" @change="onTargetChange(($event.target as HTMLSelectElement).value)">
        <option v-for="l in enabledLanguages.filter(l => l.code !== 'auto')" :key="l.code" :value="l.code">{{ l.name }}</option>
      </select>
    </div>

    <div class="styles">
      <button
        v-for="opt in STYLE_OPTIONS" :key="opt.value"
        class="style-pill" :class="{ active: style === opt.value }"
        @click="emit('update:style', opt.value)"
      >{{ opt.label }}</button>
    </div>

    <button class="btn-ghost gear" @click="store.toggleDarkMode()" :title="effectiveDark ? '日间模式' : '夜间模式'">
      <svg v-if="effectiveDark" width="15" height="15" viewBox="0 0 24 24" fill="none">
        <circle cx="12" cy="12" r="5" stroke="currentColor" stroke-width="1.5"/>
        <path d="M12 2v2m0 16v2M4.93 4.93l1.41 1.41m11.32 11.32l1.41 1.41M2 12h2m16 0h2M4.93 19.07l1.41-1.41m11.32-11.32l1.41-1.41" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
      </svg>
      <svg v-else width="15" height="15" viewBox="0 0 24 24" fill="none">
        <path d="M21 12.79A9 9 0 1111.21 3a7 7 0 009.79 9.79z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>

    <button class="btn-ghost gear" @click="emit('openSettings')" title="设置">
      <svg width="15" height="15" viewBox="0 0 20 20" fill="none">
        <path d="M10 12.5a2.5 2.5 0 100-5 2.5 2.5 0 000 5z" stroke="currentColor" stroke-width="1.4"/>
        <path d="M16.17 12.5a1.39 1.39 0 00.28 1.53l.05.05a1.69 1.69 0 01-1.19 2.88 1.69 1.69 0 01-1.19-.5l-.05-.05a1.39 1.39 0 00-1.53-.28 1.39 1.39 0 00-.84 1.27v.14a1.69 1.69 0 01-3.38 0v-.08a1.39 1.39 0 00-.91-1.27 1.39 1.39 0 00-1.53.28l-.05.05a1.69 1.69 0 01-2.88-1.19c0-.45.18-.88.5-1.19l.05-.05a1.39 1.39 0 00.28-1.53 1.39 1.39 0 00-1.27-.84h-.14a1.69 1.69 0 010-3.38h.08a1.39 1.39 0 001.27-.91 1.39 1.39 0 00-.28-1.53l-.05-.05a1.69 1.69 0 012.38-2.38l.05.05a1.39 1.39 0 001.53.28h.07a1.39 1.39 0 00.84-1.27v-.14a1.69 1.69 0 013.38 0v.08a1.39 1.39 0 00.84 1.27 1.39 1.39 0 001.53-.28l.05-.05a1.69 1.69 0 012.38 2.38l-.05.05a1.39 1.39 0 00-.28 1.53v.07a1.39 1.39 0 001.27.84h.14a1.69 1.69 0 010 3.38h-.08a1.39 1.39 0 00-1.27.84z" stroke="currentColor" stroke-width="1.4"/>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 16px;
  padding-top: 10px;
  background: var(--surface);
  border-bottom: 1px solid var(--border-light);
  user-select: none;
  -webkit-app-region: drag;
}
.toolbar select, .toolbar button { -webkit-app-region: no-drag; }

.lang-group { display: flex; align-items: center; gap: 6px; }
.lang-group select {
  font-size: 12px;
  padding: 4px 26px 4px 8px;
  min-width: 72px;
  border-radius: 6px;
}

.swap {
  display: flex; align-items: center; justify-content: center;
  width: 26px; height: 26px; border-radius: 6px;
  background: transparent; color: var(--text-3);
  transition: all var(--transition);
}
.swap:hover { color: var(--accent-text); background: var(--accent-glow); }

.styles { display: flex; gap: 1px; flex: 1; }
.style-pill {
  padding: 3px 10px; border-radius: 5px;
  background: transparent; color: var(--text-3);
  font-size: 12px; font-weight: 500;
  transition: all var(--transition);
}
.style-pill:hover { color: var(--text-2); }
.style-pill.active { background: var(--surface-active); color: var(--text); }

.gear {
  margin-left: auto;
  display: flex; align-items: center; justify-content: center;
  width: 28px; height: 28px; border-radius: 6px;
  color: var(--text-3);
}
.gear:hover { color: var(--text-2); }
</style>
