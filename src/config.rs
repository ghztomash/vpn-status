use crate::args::Args;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::path::PathBuf;

// Struct to hold configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub enabled_string: Option<String>,
    pub enabled_style: Option<StyleConfig>,
    pub disabled_string: Option<String>,
    pub disabled_style: Option<StyleConfig>,
    pub lookup: Option<bool>,
    pub lookup_providers: Option<Vec<String>>,
}

// single struct to hold configuration and arguments
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StyleConfig {
    pub color: String,
    pub format: Option<Vec<String>>,
}

impl StyleConfig {
    pub fn new(color: &str) -> Self {
        Self {
            color: color.to_string(),
            format: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled_string: Some("enabled".to_string()),
            enabled_style: Some(StyleConfig::new("green")),
            disabled_string: Some("disabled".to_string()),
            disabled_style: Some(StyleConfig::new("red")),
            lookup: None,
            lookup_providers: None,
        }
    }
}

impl Config {
    fn load_config(path: Option<PathBuf>) -> Result<Self> {
        let config: Config = match path {
            Some(path) => confy::load_path(path)?,
            None => confy::load("vpn_status", Some("config"))?,
        };
        let file = confy::get_configuration_file_path("vpn_status", Some("config"))?;
        println!("default configuration file is: {:#?}", file);
        Ok(config)
    }

    pub fn get(args: Args) -> Self {
        let mut config = Config::default();

        // load config file, if no path was specified in args use default path
        // if no file is found at the default path, use default values and save new config file
        if let Ok(config_file) = Config::load_config(args.config_path) {
            config = config_file;
        }

        // override config file with cli arguments
        if args.enabled_string.is_some() {
            config.enabled_string = args.enabled_string;
        }
        if let Some(enabled_color) = args.enabled_color {
            if let Some(ref mut enabled_style) = config.enabled_style {
                enabled_style.color = enabled_color;
            } else {
                config.enabled_style = Some(StyleConfig::new(&enabled_color));
            }
        }
        if args.disabled_string.is_some() {
            config.disabled_string = args.disabled_string;
        }
        if let Some(disabled_color) = args.disabled_color {
            if let Some(ref mut disabled_style) = config.disabled_style {
                disabled_style.color = disabled_color;
            } else {
                config.disabled_style = Some(StyleConfig::new(&disabled_color));
            }
        }
        if args.lookup {
            config.lookup = Some(args.lookup);
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_bool_config() {
        let config = Config::load_config(Some(PathBuf::from("configs/bool.toml"))).unwrap();
        assert_eq!(config.enabled_string, Some("true".to_string()));
        assert_eq!(config.disabled_string, Some("false".to_string()));
        dbg!(&config);
    }

    #[test]
    fn load_config() {
        let config = Config::load_config(Some(PathBuf::from("configs/unicode.toml"))).unwrap();
        assert!(config.enabled_string.is_some());
        assert!(config.disabled_string.is_some());
        assert!(config.enabled_style.is_some());
        assert!(config.disabled_style.is_some());
        assert!(config.lookup.is_some());
        dbg!(&config);
    }
}
