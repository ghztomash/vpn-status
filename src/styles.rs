use color_eyre::eyre::eyre;
use color_eyre::Result;
use colored::Styles;
use colored::*;

/// Convert a string to an array of Styles enums
pub fn styles_from_str(src: &str) -> Result<Vec<Styles>> {
    let src = src.trim().to_lowercase();
    let values = src.split_whitespace().collect::<Vec<&str>>();
    if values.is_empty() {
        return Err(eyre!("style is empty"));
    }
    let mut styles = Vec::new();

    for value in values {
        match value {
            "clear" => styles.push(Styles::Clear),
            "bold" => styles.push(Styles::Bold),
            "dimmed" => styles.push(Styles::Dimmed),
            "underline" => styles.push(Styles::Underline),
            "reversed" => styles.push(Styles::Reversed),
            "italic" => styles.push(Styles::Italic),
            "blink" => styles.push(Styles::Blink),
            "hidden" => styles.push(Styles::Hidden),
            "strikethrough" => styles.push(Styles::Strikethrough),
            _ => return Err(eyre!("unknown style: {}", value)),
        }
    }
    Ok(styles)
}

pub fn style(input: String, styles: Vec<Styles>) -> ColoredString {
    let mut input = ColoredString::from(input);
    for style in styles {
        match style {
            Styles::Clear => input = ColoredString::from(input),
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
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        let bold = styles_from_str("bold").unwrap();
        let bold = bold.first().unwrap();
        assert_eq!(*bold, Styles::Bold);
        let clear = styles_from_str("clear").unwrap();
        let clear = clear.first().unwrap();
        assert_eq!(*clear, Styles::Clear);
        let dimmed = styles_from_str("dimmed").unwrap();
        let dimmed = dimmed.first().unwrap();
        assert_eq!(*dimmed, Styles::Dimmed);
        let underline = styles_from_str("underline").unwrap();
        let underline = underline.first().unwrap();
        assert_eq!(*underline, Styles::Underline);
        let reversed = styles_from_str("reversed").unwrap();
        let reversed = reversed.first().unwrap();
        assert_eq!(*reversed, Styles::Reversed);
        let italic = styles_from_str("italic").unwrap();
        let italic = italic.first().unwrap();
        assert_eq!(*italic, Styles::Italic);
        let blink = styles_from_str("blink").unwrap();
        let blink = blink.first().unwrap();
        assert_eq!(*blink, Styles::Blink);
        let hidden = styles_from_str("hidden").unwrap();
        let hidden = hidden.first().unwrap();
        assert_eq!(*hidden, Styles::Hidden);
        let strikethrough = styles_from_str("strikethrough").unwrap();
        let strikethrough = strikethrough.first().unwrap();
        assert_eq!(*strikethrough, Styles::Strikethrough);
    }

    #[test]
    fn convert_multiple() {
        let styles = styles_from_str("bold underline").unwrap();
        assert_eq!(styles.len(), 2);
        let bold = styles.first().unwrap();
        assert_eq!(*bold, Styles::Bold);
        let underline = styles.last().unwrap();
        assert_eq!(*underline, Styles::Underline);
    }

    #[test]
    fn error() {
        let result = styles_from_str("unknown");
        assert!(result.is_err(), "expected an error");
    }
}
