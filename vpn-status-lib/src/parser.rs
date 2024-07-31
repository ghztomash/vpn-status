use crate::styles;
use log::debug;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Syntax {
    Status,
    Ip,
    City,
    Country,
    String(String),
}

impl FromStr for Syntax {
    type Err = ();
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "status" => Ok(Self::Status),
            "ip" => Ok(Self::Ip),
            "city" => Ok(Self::City),
            "country" => Ok(Self::Country),
            _ => Ok(Self::String(str.to_string())),
        }
    }
}

/// Lookup struct
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Lookup {
    pub ip: String,
    pub city: String,
    pub country: String,
}

/// Parse output_format into syntax tokens
pub fn parse(format: &str) -> Vec<Syntax> {
    let tokens: Vec<&str> = format.split_terminator(&['{', '}']).collect();
    let mut output: Vec<Syntax> = Vec::new();

    for token in tokens {
        if !token.is_empty() {
            output.push(Syntax::from_str(token).unwrap());
        }
    }
    debug!("output_format: {:?}", output);
    output
}

/// Constructs an output string with given format
pub fn make_output(input: Vec<Syntax>, status: &str, lookup: Option<Lookup>) -> String {
    let mut output = String::new();
    let lookup = lookup.unwrap_or_default();

    for i in input {
        match i {
            Syntax::Status => output = format!("{}{}", output, status),
            Syntax::Ip => output = format!("{}{}", output, lookup.ip),
            Syntax::City => output = format!("{}{}", output, lookup.city),
            Syntax::Country => output = format!("{}{}", output, lookup.country),
            Syntax::String(s) => output = format!("{}{}", output, s),
        }
    }
    output
}

/// Constructs an output string with given format, where the static strings are styled
pub fn make_output_styled(
    input: Vec<Syntax>,
    status: &str,
    lookup: Option<Lookup>,
    style: Vec<String>,
    color: &str,
) -> String {
    let mut output = String::new();
    let lookup = lookup.unwrap_or_default();

    for i in input {
        match i {
            Syntax::Status => output = format!("{}{}", output, status),
            Syntax::Ip => output = format!("{}{}", output, lookup.ip),
            Syntax::City => output = format!("{}{}", output, lookup.city),
            Syntax::Country => output = format!("{}{}", output, lookup.country),
            Syntax::String(s) => {
                output = format!("{}{}", output, styles::apply_style(s, style.clone(), color))
            }
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
        let out = make_output(tokens, status, None);
        assert_eq!(out, format!("VPN is {status}."));
    }

    #[test]
    fn parse_status_with_lookup() {
        let format = "VPN is {status}. From: {city}, {country}.";
        let expected_tokens = vec![
            Syntax::String("VPN is ".to_string()),
            Syntax::Status,
            Syntax::String(". From: ".to_string()),
            Syntax::City,
            Syntax::String(", ".to_string()),
            Syntax::Country,
            Syntax::String(".".to_string()),
        ];

        let lookup = Lookup {
            ip: "1.1.1.1".to_string(),
            city: "City".to_string(),
            country: "Country".to_string(),
        };

        let tokens = parse(format);
        assert_eq!(tokens, expected_tokens);
        let status = "enabled";
        let out = make_output(tokens, status, Some(lookup.clone()));
        assert_eq!(
            out,
            format!(
                "VPN is {status}. From: {}, {}.",
                lookup.city, lookup.country
            )
        );
    }

    #[test]
    fn parse_status_only() {
        let format = "{status}";
        let expected_tokens = vec![Syntax::Status];
        let tokens = parse(format);
        assert_eq!(tokens, expected_tokens);
        let status = "enabled";
        let out = make_output(tokens, status, None);
        assert_eq!(out, format!("{status}"));
    }

    #[test]
    fn parse_bad() {
        let format = "{}{unknown}";
        let expected_tokens = vec![Syntax::String("unknown".to_string())];
        let tokens = parse(format);
        assert_eq!(tokens, expected_tokens);
        let out = make_output(tokens, "", None);
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
