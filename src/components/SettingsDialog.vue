<script setup lang="ts">
import { ref, computed } from "vue";
import { useSettingsStore } from "../stores/settings";
import { ALL_LANGUAGES } from "../types";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{ close: [] }>();
const store = useSettingsStore();
const activeTab = ref<"model" | "language">("model");

const otherLanguages = computed(() =>
  ALL_LANGUAGES.filter((l) => l.code !== "auto" && l.code !== "zh" && l.code !== "en")
);

function toggleLanguage(code: string) {
  const list = store.settings.enabledLanguages;
  const idx = list.indexOf(code);
  if (idx >= 0) list.splice(idx, 1);
  else list.push(code);
}

const importExportMsg = ref("");

async function handleExport() {
  try {
    const data = await invoke<string>("export_settings");
    await navigator.clipboard.writeText(data);
    importExportMsg.value = "已复制到剪贴板";
  } catch (e: any) {
    importExportMsg.value = e.toString();
  }
  setTimeout(() => (importExportMsg.value = ""), 3000);
}

async function handleImport() {
  try {
    const data = await navigator.clipboard.readText();
    await invoke("import_settings", { data });
    store.load();
    importExportMsg.value = "已导入";
  } catch (e: any) {
    importExportMsg.value = e.toString();
  }
  setTimeout(() => (importExportMsg.value = ""), 3000);
}
</script>

<template>
  <div class="dialog-overlay" @click.self="emit('close')">
    <div class="dialog">
      <div class="dialog-title">设置</div>

      <div class="tabs">
        <button
          class="tab" :class="{ active: activeTab === 'model' }"
          @click="activeTab = 'model'"
        >模型</button>
        <button
          class="tab" :class="{ active: activeTab === 'language' }"
          @click="activeTab = 'language'"
        >语言</button>
      </div>

      <div v-if="activeTab === 'model'">
        <div class="form-group">
          <label class="form-label">协议</label>
          <select v-model="store.settings.model.provider" class="form-input">
            <option value="openai">OpenAI 兼容</option>
            <option value="anthropic">Anthropic 兼容</option>
          </select>
        </div>
        <div class="form-group">
          <label class="form-label">API 地址</label>
          <input v-model="store.settings.model.baseUrl" class="form-input" placeholder="https://api.openai.com/v1" />
        </div>
        <div class="form-group">
          <label class="form-label">API Key</label>
          <input v-model="store.settings.model.apiKey" type="password" class="form-input" placeholder="sk-..." />
        </div>
        <div class="form-group">
          <label class="form-label">模型</label>
          <input v-model="store.settings.model.model" class="form-input" placeholder="gpt-4o-mini" />
        </div>
      </div>

      <div v-if="activeTab === 'language'">
        <p class="hint">中文和英文始终可用。</p>
        <div class="chip-group">
          <span class="chip active" style="opacity:0.5;cursor:default">中文</span>
          <span class="chip active" style="opacity:0.5;cursor:default">英文</span>
          <button
            v-for="l in otherLanguages" :key="l.code"
            class="chip" :class="{ active: store.settings.enabledLanguages.includes(l.code) }"
            @click="toggleLanguage(l.code)"
          >{{ l.name }}</button>
        </div>
      </div>

      <div class="foot">
        <div class="foot-left">
          <button class="btn-small" @click="handleExport">导出配置</button>
          <button class="btn-small" @click="handleImport">导入配置</button>
          <span v-if="importExportMsg" class="foot-msg">{{ importExportMsg }}</span>
        </div>
        <button class="btn-primary" @click="emit('close')">完成</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.hint {
  color: var(--text-3);
  font-size: 12px;
  margin-bottom: 12px;
}
.foot {
  margin-top: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.foot-left {
  display: flex;
  gap: 8px;
  align-items: center;
}
.foot-msg {
  font-size: 12px;
  color: var(--accent);
}
.btn-small {
  font-size: 12px;
  padding: 4px 10px;
  border-radius: 6px;
  background: var(--bg-1);
  border: 1px solid var(--border);
  color: var(--text-2);
  cursor: pointer;
}
.btn-small:hover {
  background: var(--bg-2);
  color: var(--text-1);
}
</style>
