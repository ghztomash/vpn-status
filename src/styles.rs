use colored::Styles;
use color_eyre::Result;
use color_eyre::eyre::eyre;

/// Convert a string to a Styles enum
fn style_from_str(src: &str) -> Result<Styles> {
    let src = src.to_lowercase();

    match src.as_ref() {
        "clear" => Ok(Styles::Clear),
        "bold" => Ok(Styles::Bold),
        "dimmed" => Ok(Styles::Dimmed),
        "underline" => Ok(Styles::Underline),
        "reversed" => Ok(Styles::Reversed),
        "italic" => Ok(Styles::Italic),
        "blink" => Ok(Styles::Blink),
        "hidden" => Ok(Styles::Hidden),
        "strikethrough" => Ok(Styles::Strikethrough),
        _ => Err(eyre!("unknown style: {}", src)),
    }
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
    fn error() {
        let result = style_from_str("unknown");
        assert!(result.is_err(), "expected an error");
    }
}
