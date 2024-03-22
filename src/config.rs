use crate::args::Args;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::path::PathBuf;

// single struct to hold configuration and arguments
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub enabled_string: Option<String>,
    pub enabled_color: Option<String>,
    pub enabled_style: Option<String>, // emphasys
    pub disabled_string: Option<String>,
    pub disabled_color: Option<String>,
    pub disabled_style: Option<String>,
    pub config_path: Option<PathBuf>,
    pub lookup: Option<bool>,
    pub lookup_provider: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled_string: Some("enabled".to_string()),
            enabled_color: Some("green".to_string()),
            enabled_style: None,
            disabled_string: Some("disabled".to_string()),
            disabled_color: Some("red".to_string()),
            disabled_style: None,
            config_path: None,
            lookup: None,
            lookup_provider: None,
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

    pub fn get() -> Self {
        let mut config = Config::default();
        // parse cli arguments
        let args = Args::parse_args();

        // load config file, if no path was specified in args use default path
        // if no file is found at the default path, use default values and save new config file
        if let Ok(config_file) = Config::load_config(args.config_path) {
            config = config_file;
        }

        // override config file with cli arguments
        if args.enabled_string.is_some() {
            config.enabled_string = args.enabled_string;
        }
        if args.enabled_color.is_some() {
            config.enabled_color = args.enabled_color;
        }
        if args.disabled_string.is_some() {
            config.disabled_string = args.disabled_string;
        }
        if args.disabled_color.is_some() {
            config.disabled_color = args.disabled_color;
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
    fn load_config() {
        let config = Config::load_config(Some(PathBuf::from("configs/bool.toml"))).unwrap();
        assert_eq!(config.enabled_string, Some("true".to_string()));
        assert_eq!(config.disabled_string, Some("false".to_string()));
    }
}
