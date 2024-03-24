use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Syntax {
    Status,
    String(String),
}

impl FromStr for Syntax {
    type Err = ();
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "status" => Ok(Self::Status),
            _ => Ok(Self::String(str.to_string())),
        }
    }
}

/// Parse output_format into syntax tokens
pub fn parse(format: &str) -> Vec<Syntax> {
    let tokens: Vec<&str> = format.split_terminator(&['{', '}']).collect();
    let mut output: Vec<Syntax> = Vec::new();
    // TODO: remove
    dbg!(&tokens);

    for token in tokens {
        if !token.is_empty() {
            output.push(Syntax::from_str(token).unwrap());
        }
    }
    // TODO: remove
    dbg!(&output);
    output
}

/// Constructs an output string with given format
pub fn make_output(input: Vec<Syntax>, status: &str) -> String {
    let mut output = String::new();
    for i in input {
        match i {
            Syntax::Status => output = format!("{}{}", output, status),
            Syntax::String(s) => output = format!("{}{}", output, s),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_status_with_text() {
        let format = "VPN is {status}.";
        let expected_tokens = vec![
            Syntax::String("VPN is ".to_string()),
            Syntax::Status,
            Syntax::String(".".to_string()),
        ];
        let tokens = parse(format);
        assert_eq!(tokens, expected_tokens);
        let status = "enabled";
        let out = make_output(tokens, status);
        assert_eq!(out, format!("VPN is {status}."));
    }

    #[test]
    fn parse_status_only() {
        let format = "{status}";
        let expected_tokens = vec![Syntax::Status];
        let tokens = parse(format);
        assert_eq!(tokens, expected_tokens);
        let status = "enabled";
        let out = make_output(tokens, status);
        assert_eq!(out, format!("{status}"));
    }

    #[test]
    fn parse_bad() {
        let format = "{}{unknown}";
        let expected_tokens = vec![Syntax::String("unknown".to_string())];
        let tokens = parse(format);
        assert_eq!(tokens, expected_tokens);
        let out = make_output(tokens, "");
        assert_eq!(out, String::from("unknown"));
    }

    #[test]
    fn parse_empty() {
        let format = "{}";
        let expected_tokens = vec![];
        let tokens = parse(format);
        assert_eq!(tokens, expected_tokens);
    }
}
