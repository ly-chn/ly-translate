use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Instant;

static CLIENT: std::sync::LazyLock<reqwest::Client> = std::sync::LazyLock::new(|| {
    reqwest::Client::builder()
        .pool_max_idle_per_host(2)
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("ly-translate/1.0")
        .build()
        .unwrap()
});

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WordDefinition {
    pub word: String,
    pub phonetic: String,
    #[serde(rename = "ukPhonetic", default)]
    pub uk_phonetic: String,
    #[serde(rename = "usPhonetic", default)]
    pub us_phonetic: String,
    #[serde(rename = "ukSpeech", default)]
    pub uk_speech: String,
    #[serde(rename = "usSpeech", default)]
    pub us_speech: String,
    pub definitions: Vec<Definition>,
    pub examples: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Definition {
    pub pos: String,
    pub meaning: String,
}

fn encode_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for b in s.as_bytes() {
        match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn speech_url(word: &str, r#type: u8) -> String {
    format!(
        "https://dict.youdao.com/dictvoice?audio={}&type={}",
        encode_component(word),
        r#type
    )
}

pub async fn lookup(word: &str, _lang: &str, _config: &crate::config::ModelConfig) -> Result<WordDefinition> {
    let w = word.trim();
    if w.is_empty() {
        anyhow::bail!("empty word");
    }

    println!("[dict] youdao lookup '{}' ...", w);
    let start = Instant::now();

    let resp = CLIENT
        .get("https://dict.youdao.com/jsonapi")
        .query(&[
            ("jsonversion", "2"),
            ("client", "mobile"),
            ("q", w),
        ])
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err = resp.text().await.unwrap_or_default();
        anyhow::bail!("youdao error {}: {}", status, err);
    }

    let data: Value = resp.json().await?;
    let def = parse_youdao(w, &data);

    println!(
        "[dict] done: {}ms ({} senses)",
        start.elapsed().as_millis(),
        def.definitions.len()
    );

    if def.definitions.is_empty() && def.phonetic.is_empty() {
        anyhow::bail!("no dictionary entry");
    }

    Ok(def)
}

fn parse_youdao(query: &str, data: &Value) -> WordDefinition {
    let mut word = query.to_string();
    let mut uk_phonetic = String::new();
    let mut us_phonetic = String::new();
    let mut definitions: Vec<Definition> = Vec::new();
    let mut examples: Vec<String> = Vec::new();

    // simple.word[0]
    if let Some(arr) = data.pointer("/simple/word").and_then(|v| v.as_array()) {
        if let Some(sw) = arr.first() {
            if let Some(p) = first_str(sw, &["return-phrase", "return_phrase"]) {
                word = p.to_string();
            }
            uk_phonetic = first_str(sw, &["ukphone", "ukPhone"]).unwrap_or("").to_string();
            us_phonetic = first_str(sw, &["usphone", "usPhone"]).unwrap_or("").to_string();
        }
    }

    // ec (英汉)
    parse_ec(data.get("ec"), &mut word, &mut uk_phonetic, &mut us_phonetic, &mut definitions);

    // ce (汉英) — 若还没有释义
    if definitions.is_empty() {
        parse_ce(data.get("ce"), &mut word, &mut definitions);
    }

    // 新版 / 备用字段
    if definitions.is_empty() {
        parse_new_ec(data, &mut uk_phonetic, &mut us_phonetic, &mut definitions);
    }

    // 网络释义兜底
    if definitions.is_empty() {
        parse_web_trans(data.get("web_trans"), &mut definitions);
    }

    // 双语例句
    parse_examples(data, &mut examples);

    let phonetic = if !uk_phonetic.is_empty() && !us_phonetic.is_empty() {
        if uk_phonetic == us_phonetic {
            uk_phonetic.clone()
        } else {
            format!("英 {uk_phonetic}  美 {us_phonetic}")
        }
    } else if !uk_phonetic.is_empty() {
        uk_phonetic.clone()
    } else {
        us_phonetic.clone()
    };

    let speech_key = word.clone();
    WordDefinition {
        word,
        phonetic,
        uk_phonetic,
        us_phonetic,
        uk_speech: speech_url(&speech_key, 1),
        us_speech: speech_url(&speech_key, 2),
        definitions,
        examples,
    }
}

fn first_str<'a>(v: &'a Value, keys: &[&str]) -> Option<&'a str> {
    for k in keys {
        if let Some(s) = v.get(*k).and_then(|x| x.as_str()) {
            if !s.is_empty() {
                return Some(s);
            }
        }
    }
    None
}

fn parse_ec(
    ec: Option<&Value>,
    word: &mut String,
    uk: &mut String,
    us: &mut String,
    defs: &mut Vec<Definition>,
) {
    let Some(ec) = ec else { return };

    // word 可能是对象或数组
    let word_node = match ec.get("word") {
        Some(Value::Array(arr)) => arr.first(),
        Some(obj) => Some(obj),
        None => None,
    };
    let Some(wn) = word_node else { return };

    if let Some(p) = first_str(wn, &["return-phrase", "return_phrase", "hw"]) {
        *word = p.to_string();
    }
    if uk.is_empty() {
        *uk = first_str(wn, &["ukphone", "ukPhone"]).unwrap_or("").to_string();
    }
    if us.is_empty() {
        *us = first_str(wn, &["usphone", "usPhone"]).unwrap_or("").to_string();
    }

    // 形态1: trs[].pos + trs[].tran
    if let Some(trs) = wn.get("trs").and_then(|t| t.as_array()) {
        for tr in trs {
            if let Some(tran) = tr.get("tran").and_then(|t| t.as_str()) {
                let pos = tr.get("pos").and_then(|p| p.as_str()).unwrap_or("").to_string();
                push_def(defs, &pos, tran);
                continue;
            }
            // 形态2: trs[].tr[].l.i[]  (数组里可能混了 pos 与释义)
            if let Some(tr_list) = tr.get("tr").and_then(|t| t.as_array()) {
                for item in tr_list {
                    if let Some(parts) = item.pointer("/l/i").and_then(|i| i.as_array()) {
                        let texts: Vec<&str> = parts
                            .iter()
                            .filter_map(|p| p.as_str())
                            .filter(|s| !s.is_empty() && *s != ".")
                            .collect();
                        if texts.is_empty() {
                            continue;
                        }
                        let joined = texts.join("");
                        let (pos, meaning) = split_pos_meaning(&joined);
                        push_def(defs, &pos, &meaning);
                    } else if let Some(s) = item.pointer("/l/i").and_then(|i| i.as_str()) {
                        let (pos, meaning) = split_pos_meaning(s);
                        push_def(defs, &pos, &meaning);
                    }
                }
            }
        }
    }
}

fn parse_ce(ce: Option<&Value>, word: &mut String, defs: &mut Vec<Definition>) {
    let Some(ce) = ce else { return };
    let word_node = match ce.get("word") {
        Some(Value::Array(arr)) => arr.first(),
        Some(obj) => Some(obj),
        None => None,
    };
    let Some(wn) = word_node else { return };

    if let Some(p) = first_str(wn, &["return-phrase", "return_phrase"]) {
        *word = p.to_string();
    }

    if let Some(trs) = wn.get("trs").and_then(|t| t.as_array()) {
        for tr in trs {
            // #text / tr / l
            if let Some(t) = tr.get("#text").and_then(|x| x.as_str()) {
                push_def(defs, "", t);
                continue;
            }
            if let Some(tran) = tr.get("tran").and_then(|x| x.as_str()) {
                let pos = tr.get("pos").and_then(|p| p.as_str()).unwrap_or("");
                push_def(defs, pos, tran);
                continue;
            }
            if let Some(tr_list) = tr.get("tr").and_then(|t| t.as_array()) {
                for item in tr_list {
                    if let Some(s) = item.pointer("/l/i").and_then(|i| i.as_str()) {
                        push_def(defs, "", s);
                    } else if let Some(arr) = item.pointer("/l/i").and_then(|i| i.as_array()) {
                        let s: String = arr.iter().filter_map(|x| x.as_str()).collect();
                        if !s.is_empty() {
                            push_def(defs, "", &s);
                        }
                    }
                }
            }
        }
    }
}

fn parse_new_ec(
    data: &Value,
    uk: &mut String,
    us: &mut String,
    defs: &mut Vec<Definition>,
) {
    // 部分响应在 ec.word.phone / 扁平字段
    if let Some(basic) = data.pointer("/basic") {
        if uk.is_empty() {
            *uk = first_str(basic, &["uk-phonetic", "ukPhonetic", "phonetic"]).unwrap_or("").to_string();
        }
        if us.is_empty() {
            *us = first_str(basic, &["us-phonetic", "usPhonetic"]).unwrap_or("").to_string();
        }
        if let Some(explains) = basic.get("explains").and_then(|e| e.as_array()) {
            for e in explains {
                if let Some(s) = e.as_str() {
                    let (pos, meaning) = split_pos_meaning(s);
                    push_def(defs, &pos, &meaning);
                }
            }
        }
    }
}

fn parse_web_trans(web: Option<&Value>, defs: &mut Vec<Definition>) {
    let Some(web) = web else { return };
    let list = web
        .get("web-translation")
        .or_else(|| web.get("web_translation"))
        .and_then(|v| v.as_array());
    let Some(list) = list else { return };

    for item in list.iter().take(6) {
        let key = item.get("@key").or_else(|| item.get("key")).and_then(|k| k.as_str());
        let trans = item
            .get("trans")
            .and_then(|t| t.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| t.get("value").or_else(|| t.get("#text")).and_then(|v| v.as_str()))
                    .collect::<Vec<_>>()
                    .join("；")
            })
            .unwrap_or_default();
        if trans.is_empty() {
            continue;
        }
        let meaning = if let Some(k) = key {
            if k.is_empty() {
                trans
            } else {
                format!("{k}: {trans}")
            }
        } else {
            trans
        };
        push_def(defs, "网络", &meaning);
    }
}

fn parse_examples(data: &Value, examples: &mut Vec<String>) {
    // blng_sents_part.sentence-pair
    if let Some(pairs) = data
        .pointer("/blng_sents_part/sentence-pair")
        .and_then(|v| v.as_array())
    {
        for p in pairs.iter().take(3) {
            let en = p
                .get("sentence")
                .or_else(|| p.get("sentence-eng"))
                .and_then(|s| s.as_str())
                .unwrap_or("")
                .trim();
            let zh = p
                .get("sentence-translation")
                .or_else(|| p.get("sentence-chn"))
                .and_then(|s| s.as_str())
                .unwrap_or("")
                .trim();
            if !en.is_empty() && !zh.is_empty() {
                examples.push(format!("{en}\n{zh}"));
            } else if !en.is_empty() {
                examples.push(en.to_string());
            }
        }
    }

    if examples.len() >= 3 {
        return;
    }

    // auth_sents_part
    if let Some(sents) = data
        .pointer("/auth_sents_part/sent")
        .and_then(|v| v.as_array())
    {
        for s in sents.iter().take(3 - examples.len()) {
            if let Some(f) = s.get("foreign").and_then(|x| x.as_str()) {
                let clean = strip_tags(f);
                if !clean.is_empty() {
                    examples.push(clean);
                }
            }
        }
    }
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn split_pos_meaning(s: &str) -> (String, String) {
    let s = s.trim();
    // "n. 账户；解释" / "int. 你好"
    let re_pos = s.find(". ");
    if let Some(idx) = re_pos {
        let pos = s[..=idx].trim().to_string();
        // 词性一般较短
        if pos.len() <= 12 && pos.chars().all(|c| c.is_ascii() || c == '.') {
            return (pos, s[idx + 2..].trim().to_string());
        }
    }
    // "n.账户"
    if let Some(idx) = s.find('.') {
        if idx > 0 && idx <= 8 {
            let pos = s[..=idx].trim().to_string();
            if pos.chars().all(|c| c.is_ascii() || c == '.') {
                return (pos, s[idx + 1..].trim().to_string());
            }
        }
    }
    (String::new(), s.to_string())
}

fn push_def(defs: &mut Vec<Definition>, pos: &str, meaning: &str) {
    let meaning = meaning.trim();
    if meaning.is_empty() {
        return;
    }
    // 去重
    if defs.iter().any(|d| d.meaning == meaning && d.pos == pos) {
        return;
    }
    defs.push(Definition {
        pos: pos.trim().to_string(),
        meaning: meaning.to_string(),
    });
}
