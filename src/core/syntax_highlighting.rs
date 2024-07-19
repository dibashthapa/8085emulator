use eframe::egui::{text::LayoutJob, Color32, TextFormat};

#[derive(Debug)]
pub struct Language {
    keywords: std::collections::BTreeSet<&'static str>,
}

impl Language {
    pub fn new() -> Self {
        Self {
            keywords: [
                "mov", "mvi", "lxi", "lda", "sta", "lhld", "shld", "stax", "xchg", "add", "adi",
                "sub", "sui", "inr", "dcr", "inx", "dcx", "dad", "ana", "ani", "ora", "ori", "xra",
                "xri", "cma", "rlc", "rrc", "ral", "rar", "push", "pop", "jmp", "jnz", "jz", "jnc",
                "jc", "call", "ret", "rst", "pchl", "sphl", "xthl", "out", "in", "ei", "di", "rim",
                "sim", "hlt", "nop",
            ]
            .into_iter()
            .collect(),
        }
    }

    fn is_keyword(&self, word: &str) -> bool {
        self.keywords.contains(word)
    }
}

pub fn highlight(mut text: &str) -> LayoutJob {
    let language = Language::new();
    let mut job = LayoutJob::default();
    let font_id = eframe::egui::FontId::monospace(17.0);
    // TokenType::Comment => TextFormat::simple(font_id.clone(), Color32::from_gray(120)),
    // TokenType::Keyword => TextFormat::simple(font_id.clone(), Color32::from_rgb(255, 100, 100)),
    // TokenType::Literal => TextFormat::simple(font_id.clone(), Color32::from_rgb(87, 165, 171)),
    // TokenType::StringLiteral => TextFormat::simple(font_id.clone(), Color32::from_rgb(109, 147, 226)),
    // TokenType::Punctuation => TextFormat::simple(font_id.clone(), Color32::LIGHT_GRAY),
    // TokenType::Whitespace => TextFormat::simple(font_id.clone(), Color32::TRANSPARENT),

    while !text.is_empty() {
        if text.starts_with(";") {
            let end = text.find("\n").unwrap_or(text.len());
            job.append(
                &text[..end],
                0.0,
                TextFormat::simple(font_id.clone(), Color32::from_gray(120)),
            );
            text = &text[end..];
        } else if text.starts_with(|c: char| c.is_ascii_alphanumeric()) {
            let end = text[1..]
                .find(|c: char| !c.is_ascii_alphanumeric())
                .map_or_else(|| text.len(), |i| i + 1);
            let word = &text[..end];
            if language.is_keyword(&word.to_lowercase()) {
                job.append(
                    word,
                    0.0,
                    TextFormat::simple(font_id.clone(), Color32::from_rgb(109, 147, 226)),
                );
            } else {
                job.append(
                    word,
                    0.0,
                    TextFormat::simple(font_id.clone(), Color32::from_rgb(255, 100, 100)),
                );
            }
            text = &text[end..];
        } else if text.starts_with(|c: char| c.is_ascii_whitespace()) {
            let end = text[1..]
                .find(|c: char| !c.is_ascii_whitespace())
                .map_or_else(|| text.len(), |i| i + 1);
            job.append(
                &text[..end],
                0.0,
                TextFormat::simple(font_id.clone(), Color32::TRANSPARENT),
            );
            text = &text[end..];
        } else {
            let mut it = text.char_indices();
            it.next();
            let end = it.next().map_or(text.len(), |(idx, _chr)| idx);
            job.append(
                &text[..end],
                0.0,
                TextFormat::simple(font_id.clone(), Color32::LIGHT_GRAY),
            );
            text = &text[end..];
        }
    }
    job
}
