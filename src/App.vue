<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useSettingsStore } from "./stores/settings";
import LanguageBar from "./components/LanguageBar.vue";
import TranslatePanel from "./components/TranslatePanel.vue";
import SettingsDialog from "./components/SettingsDialog.vue";
import { invoke } from "@tauri-apps/api/core";

const store = useSettingsStore();
const showSettings = ref(false);
const sourceLang = ref("auto");
const targetLang = ref("en");
const style = ref<"professional_ecommerce" | "general" | "marketing" | "casual">(
  "professional_ecommerce"
);

onMounted(async () => {
  await store.load();
  invoke("init_tray");
});
</script>

<template>
  <div class="app">
    <div class="titlebar-accent"></div>
    <LanguageBar
      v-model:source-lang="sourceLang"
      v-model:target-lang="targetLang"
      v-model:style="style"
      @open-settings="showSettings = true"
    />
    <TranslatePanel
      :source-lang="sourceLang"
      :target-lang="targetLang"
      :style="style"
    />
    <SettingsDialog v-if="showSettings" @close="showSettings = false" />
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  position: relative;
}

.titlebar-accent {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, var(--accent), #a78bfa, #6e8efb);
  z-index: 10;
  opacity: 0.6;
}
</style>
