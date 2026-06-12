use anyhow::Result;
use serde_json::{json, Value};
use std::time::Instant;

use crate::config::ModelConfig;

static CLIENT: std::sync::LazyLock<reqwest::Client> = std::sync::LazyLock::new(|| {
    reqwest::Client::builder()
        .pool_max_idle_per_host(2)
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .unwrap()
});

fn style_prompt(style: &str) -> &str {
    match style {
        "professional_ecommerce" => "你是一个专业的跨境电商翻译专家，熟悉亚马逊平台术语和客服用语。翻译要准确、专业、符合电商场景。",
        "general" => "你是一个通用翻译助手，提供准确流畅的翻译。",
        "marketing" => "你是一个营销文案翻译专家，翻译时注重吸引力和转化率，语言要生动有感染力。",
        "casual" => "你是一个口语化翻译助手，翻译要自然地道，贴近目标语言的日常表达。",
        "selection" => "你是一个翻译助手，对选中的文本提供详细翻译。按以下格式输出：\n1. 翻译结果\n2. 简要解释含义和用法\n3. 如果原文在特定语境下可能引起歧义或误解，指出注意事项",
        _ => "你是一个翻译助手。",
    }
}

fn lang_name(code: &str) -> &str {
    match code {
        "zh" => "中文",
        "en" => "英文",
        "de" => "德语",
        "ja" => "日语",
        "fr" => "法语",
        "es" => "西班牙语",
        "pt" => "葡萄牙语",
        "ko" => "韩语",
        "ru" => "俄语",
        "ar" => "阿拉伯语",
        "it" => "意大利语",
        "nl" => "荷兰语",
        "pl" => "波兰语",
        "th" => "泰语",
        "vi" => "越南语",
        "id" => "印尼语",
        "tr" => "土耳其语",
        _ => code,
    }
}

pub async fn translate(
    text: &str,
    from: &str,
    to: &str,
    style: &str,
    config: &ModelConfig,
    seq: u64,
    mut cancel_rx: tokio::sync::watch::Receiver<u64>,
) -> Result<Option<String>> {
    let system = format!(
        "{} 只输出翻译结果，不要解释、不要加引号。",
        style_prompt(style)
    );
    let (src, tgt) = if from == "auto" {
        ("自动检测".to_string(), lang_name(to).to_string())
    } else {
        (lang_name(from).to_string(), lang_name(to).to_string())
    };
    let user_msg = format!("将以下{}文本翻译为{}：\n\n{}", src, tgt, text);

    println!(
        "[translate] {} {}->{} ({} chars)",
        style,
        from,
        to,
        text.len()
    );
    let start = Instant::now();

    let http_fut: std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send>> =
        match config.provider.as_str() {
            "anthropic" => Box::pin(call_anthropic(config, &system, &user_msg, 2048)),
            _ => Box::pin(call_openai(config, &system, &user_msg, 2048)),
        };

    let result = tokio::select! {
        r = http_fut => r.map(Some),
        _ = cancel_rx.wait_for(|&v| v > seq) => {
            println!("[translate] cancelled (seq {})", seq);
            Ok(None)
        }
    };

    if result.as_ref().ok().and_then(|o| o.as_ref()).is_some() {
        println!("[translate] done: {}ms", start.elapsed().as_millis());
    }
    result
}

pub async fn call_openai(
    config: &ModelConfig,
    system: &str,
    user: &str,
    max_tokens: u32,
) -> Result<String> {
    let url = format!("{}/chat/completions", config.base_url.trim_end_matches('/'));
    let body = json!({
        "model": config.model,
        "messages": [
            {"role": "system", "content": system},
            {"role": "user", "content": user}
        ],
        "temperature": 0.1,
        "max_tokens": max_tokens,
    });

    let resp = CLIENT
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_key))
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err = resp.text().await.unwrap_or_default();
        anyhow::bail!("API error {}: {}", status, err);
    }

    let data: Value = resp.json().await?;
    Ok(data["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string())
}

pub async fn call_anthropic(
    config: &ModelConfig,
    system: &str,
    user: &str,
    max_tokens: u32,
) -> Result<String> {
    let url = format!("{}/messages", config.base_url.trim_end_matches('/'));
    let body = json!({
        "model": config.model,
        "max_tokens": max_tokens,
        "system": system,
        "messages": [{"role": "user", "content": user}],
    });

    let resp = CLIENT
        .post(&url)
        .header("x-api-key", &config.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err = resp.text().await.unwrap_or_default();
        anyhow::bail!("API error {}: {}", status, err);
    }

    let data: Value = resp.json().await?;
    Ok(data["content"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string())
}
