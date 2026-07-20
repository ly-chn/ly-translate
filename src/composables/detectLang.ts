/** 轻量语种检测，用于 auto 方向纠正（非精确 NLP） */
export function detectLang(text: string): string {
  const t = text.replace(/\s+/g, "");
  if (!t) return "en";

  let cjk = 0;
  let kana = 0;
  let hangul = 0;
  let cyrillic = 0;
  let arabic = 0;
  let thai = 0;
  let latin = 0;
  const limit = Math.min(t.length, 400);

  for (let i = 0; i < limit; i++) {
    const c = t.charCodeAt(i);
    if (c >= 0x4e00 && c <= 0x9fff) cjk++;
    else if (c >= 0x3400 && c <= 0x4dbf) cjk++;
    else if (c >= 0x3040 && c <= 0x30ff) kana++;
    else if (c >= 0xac00 && c <= 0xd7af) hangul++;
    else if (c >= 0x0400 && c <= 0x04ff) cyrillic++;
    else if (c >= 0x0600 && c <= 0x06ff) arabic++;
    else if (c >= 0x0e00 && c <= 0x0e7f) thai++;
    else if ((c >= 65 && c <= 90) || (c >= 97 && c <= 122)) latin++;
  }

  if (kana > 0 && kana + cjk > latin) return "ja";
  if (hangul > 2 && hangul >= cjk) return "ko";
  if (cyrillic > latin && cyrillic > cjk) return "ru";
  if (arabic > latin && arabic > cjk) return "ar";
  if (thai > latin && thai > cjk) return "th";
  if (cjk > 0 && cjk >= Math.max(1, latin * 0.25)) return "zh";
  if (latin > 0) return "en";
  if (cjk > 0) return "zh";
  return "en";
}

/**
 * source=auto 且正文语种与目标语相同时，把目标翻到中/英另一侧，
 * 避免「英文 → 英文」。
 */
export function resolveTranslatePair(
  text: string,
  from: string,
  to: string
): { from: string; to: string } {
  if (from !== "auto") return { from, to };

  const detected = detectLang(text);
  if (detected === to) {
    return { from: "auto", to: to === "zh" ? "en" : "zh" };
  }
  return { from: "auto", to };
}
