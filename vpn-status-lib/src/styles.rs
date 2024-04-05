use crate::error::VpnStatusError;
use colored::Styles;
use colored::*;

/// Convert a string to an array of Styles enums
pub fn styles_from_vec(style_values: Vec<&str>) -> Result<Vec<Styles>, VpnStatusError> {
    if style_values.is_empty() {
        return Err(VpnStatusError::StyleError("styles are empty".to_string()));
    }
    let mut styles = Vec::new();

    for value in style_values {
        if let Ok(style) = style_from_str(value) {
            styles.push(style);
        }
    }
    Ok(styles)
}

/// Convert a string to an Styles enum
pub fn style_from_str(src: &str) -> Result<Styles, VpnStatusError> {
    let src = src.trim().to_lowercase();
    let src = src.as_str();

    match src {
        "clear" => Ok(Styles::Clear),
        "bold" => Ok(Styles::Bold),
        "dimmed" => Ok(Styles::Dimmed),
        "underline" => Ok(Styles::Underline),
        "reversed" => Ok(Styles::Reversed),
        "italic" => Ok(Styles::Italic),
        "blink" => Ok(Styles::Blink),
        "hidden" => Ok(Styles::Hidden),
        "strikethrough" => Ok(Styles::Strikethrough),
        _ => Err(VpnStatusError::StyleError(format!(
            "unknown style: {}",
            src
        ))),
    }
}

/// Applies a Style and color to an input string
pub fn apply_style(input: String, styles: Vec<String>, color: &str) -> String {
    let custom_style: Vec<&str> = styles.iter().map(|x| x.as_ref()).collect();
    let styles = styles_from_vec(custom_style).unwrap_or_default();

    let mut input = ColoredString::from(input);
    for style in styles {
        match style {
            Styles::Clear => (),
            Styles::Bold => input = input.bold(),
            Styles::Dimmed => input = input.dimmed(),
            Styles::Underline => input = input.underline(),
            Styles::Reversed => input = input.reversed(),
            Styles::Italic => input = input.italic(),
            Styles::Blink => input = input.blink(),
            Styles::Hidden => input = input.hidden(),
            Styles::Strikethrough => input = input.strikethrough(),
        }
    }
    let color = colored::Color::from(color);
    input.color(color).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        let bold = style_from_str("bold").unwrap();
        assert_eq!(bold, Styles::Bold);
        let clear = style_from_str("clear").unwrap();
        assert_eq!(clear, Styles::Clear);
        let dimmed = style_from_str("dimmed").unwrap();
        assert_eq!(dimmed, Styles::Dimmed);
        let underline = style_from_str("underline").unwrap();
        assert_eq!(underline, Styles::Underline);
        let reversed = style_from_str("reversed").unwrap();
        assert_eq!(reversed, Styles::Reversed);
        let italic = style_from_str("italic").unwrap();
        assert_eq!(italic, Styles::Italic);
        let blink = style_from_str("blink").unwrap();
        assert_eq!(blink, Styles::Blink);
        let hidden = style_from_str("hidden").unwrap();
        assert_eq!(hidden, Styles::Hidden);
        let strikethrough = style_from_str("strikethrough").unwrap();
        assert_eq!(strikethrough, Styles::Strikethrough);
    }

    #[test]
    fn convert_multiple() {
        let styles = styles_from_vec(vec!["bold", "underline"]).unwrap();
        assert_eq!(styles.len(), 2);
        let bold = styles.first().unwrap();
        assert_eq!(*bold, Styles::Bold);
        let underline = styles.last().unwrap();
        assert_eq!(*underline, Styles::Underline);
    }

    #[test]
    fn error() {
        let result = style_from_str("unknown");
        assert!(result.is_err(), "expected an error");
    }
}
