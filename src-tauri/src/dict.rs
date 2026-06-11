use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::config::ModelConfig;
use crate::translate_mod;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WordDefinition {
    pub word: String,
    pub phonetic: String,
    pub definitions: Vec<Definition>,
    pub examples: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Definition {
    pub pos: String,
    pub meaning: String,
}

pub async fn lookup(word: &str, _lang: &str, config: &ModelConfig) -> Result<WordDefinition> {
    let w = word.trim();
    println!("[dict] lookup '{}' ...", w);
    let start = Instant::now();

    let prompt = format!(
        "查询 \"{}\"。JSON: {{\"word\":\"...\",\"phonetic\":\"音标\",\"definitions\":[{{\"pos\":\"词性\",\"meaning\":\"含义\"}}],\"examples\":[\"例句\"]}}\n只输出JSON。",
        w
    );

    let result = match config.provider.as_str() {
        "anthropic" => {
            translate_mod::call_anthropic(config, "你是词典助手，只输出JSON", &prompt, 512).await
        }
        _ => translate_mod::call_openai(config, "你是词典助手，只输出JSON", &prompt, 512).await,
    };

    println!("[dict] done: {}ms", start.elapsed().as_millis());

    let text = result?;
    let json_str = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    serde_json::from_str::<WordDefinition>(json_str).or_else(|_| {
        Ok(WordDefinition {
            word: w.to_string(),
            phonetic: String::new(),
            definitions: vec![Definition {
                pos: String::new(),
                meaning: json_str.to_string(),
            }],
            examples: vec![],
        })
    })
}
