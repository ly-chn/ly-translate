use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub model: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            provider: "openai".into(),
            base_url: "https://api.openai.com/v1".into(),
            api_key: String::new(),
            model: "gpt-4o-mini".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub model: ModelConfig,
    #[serde(rename = "enabledLanguages", default = "default_languages")]
    pub enabled_languages: Vec<String>,
    #[serde(rename = "alwaysOnTop", default)]
    pub always_on_top: bool,
    #[serde(rename = "autoStart", default)]
    pub auto_start: bool,
    #[serde(rename = "darkMode", default)]
    pub dark_mode: Option<bool>,
}

fn default_languages() -> Vec<String> {
    vec![
        "zh".into(),
        "en".into(),
        "de".into(),
        "ja".into(),
        "fr".into(),
        "es".into(),
        "pt".into(),
    ]
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            model: ModelConfig::default(),
            enabled_languages: default_languages(),
            always_on_top: false,
            auto_start: false,
            dark_mode: None,
        }
    }
}
