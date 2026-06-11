export interface Language {
  code: string;
  name: string;
  nameEn: string;
}

export type TranslationStyle = "professional_ecommerce" | "general" | "marketing" | "casual";

export interface StyleOption {
  value: TranslationStyle;
  label: string;
}

export interface ModelConfig {
  provider: "openai" | "anthropic";
  baseUrl: string;
  apiKey: string;
  model: string;
}

export interface AppSettings {
  model: ModelConfig;
  enabledLanguages: string[];
  alwaysOnTop: boolean;
  autoStart: boolean;
}

export interface WordDefinition {
  word: string;
  phonetic: string;
  definitions: { pos: string; meaning: string }[];
  examples: string[];
}

export const DEFAULT_LANGUAGES: Language[] = [
  { code: "auto", name: "自动检测", nameEn: "Auto Detect" },
  { code: "zh", name: "中文", nameEn: "Chinese" },
  { code: "en", name: "英文", nameEn: "English" },
  { code: "de", name: "德语", nameEn: "German" },
  { code: "ja", name: "日语", nameEn: "Japanese" },
  { code: "fr", name: "法语", nameEn: "French" },
  { code: "es", name: "西班牙语", nameEn: "Spanish" },
  { code: "pt", name: "葡萄牙语", nameEn: "Portuguese" },
];

export const ALL_LANGUAGES: Language[] = [
  ...DEFAULT_LANGUAGES,
  { code: "ko", name: "韩语", nameEn: "Korean" },
  { code: "ru", name: "俄语", nameEn: "Russian" },
  { code: "ar", name: "阿拉伯语", nameEn: "Arabic" },
  { code: "it", name: "意大利语", nameEn: "Italian" },
  { code: "nl", name: "荷兰语", nameEn: "Dutch" },
  { code: "pl", name: "波兰语", nameEn: "Polish" },
  { code: "th", name: "泰语", nameEn: "Thai" },
  { code: "vi", name: "越南语", nameEn: "Vietnamese" },
  { code: "id", name: "印尼语", nameEn: "Indonesian" },
  { code: "tr", name: "土耳其语", nameEn: "Turkish" },
];

export const STYLE_OPTIONS: StyleOption[] = [
  { value: "professional_ecommerce", label: "专业电商" },
  { value: "general", label: "通用翻译" },
  { value: "marketing", label: "营销文案" },
  { value: "casual", label: "口语自然" },
];
