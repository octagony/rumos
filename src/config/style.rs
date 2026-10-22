use colored::{Color, Colorize};
use std::str::FromStr;

use crate::constants::{FONT_STYLE_TYPE_BOLD, FONT_STYLE_TYPE_ITALIC, FONT_STYLE_TYPE_UNDERLINE};

pub fn apply_style(text: &str, color_str: &Option<String>, style_str: &Option<String>) -> String {
    let color = color_str
        .as_ref()
        .and_then(|c| Color::from_str(c).ok())
        .unwrap_or(Color::Black);

    let mut colored_text = text.color(color);

    if let Some(style_str) = style_str {
        colored_text = match style_str.to_lowercase().as_str() {
            FONT_STYLE_TYPE_BOLD => colored_text.bold(),
            FONT_STYLE_TYPE_ITALIC => colored_text.italic(),
            FONT_STYLE_TYPE_UNDERLINE => colored_text.underline(),
            _ => colored_text,
        };
    }

    colored_text.to_string()
}
