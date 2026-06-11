import { defineStore } from "pinia";
import { ref, watch } from "vue";
import type { AppSettings, TranslationStyle } from "../types";
import { invoke } from "@tauri-apps/api/core";

const DEFAULT_SETTINGS: AppSettings = {
  model: {
    provider: "openai",
    baseUrl: "https://api.openai.com/v1",
    apiKey: "",
    model: "gpt-4o-mini",
  },
  enabledLanguages: ["zh", "en", "de", "ja", "fr", "es", "pt"],
  alwaysOnTop: false,
  autoStart: false,
};

export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<AppSettings>({ ...DEFAULT_SETTINGS });
  const style = ref<TranslationStyle>("professional_ecommerce");
  const settingsLoaded = ref(false);

  const systemDark = window.matchMedia("(prefers-color-scheme: dark)");
  systemDark.addEventListener("change", () => {
    if (settings.value.darkMode === undefined) applyTheme();
  });

  function applyTheme() {
    const isDark =
      settings.value.darkMode ?? systemDark.matches;
    document.documentElement.classList.toggle("light", !isDark);
  }

  async function load() {
    try {
      const saved = await invoke<AppSettings | null>("load_settings");
      if (saved) {
        settings.value = {
          ...DEFAULT_SETTINGS,
          ...saved,
          model: { ...DEFAULT_SETTINGS.model, ...saved.model },
        };
      }
    } catch (e) {
      console.error("Failed to load settings:", e);
    }
    settingsLoaded.value = true;
    applyTheme();
  }

  async function save() {
    try {
      await invoke("save_settings", { settings: settings.value });
    } catch (e) {
      console.error("Failed to save settings:", e);
    }
  }

  watch(settings, save, { deep: true });
  watch(() => settings.value.darkMode, applyTheme);

  function toggleDarkMode() {
    const effective = settings.value.darkMode ?? systemDark.matches;
    settings.value.darkMode = !effective;
  }

  return { settings, style, settingsLoaded, load, save, toggleDarkMode };
});
