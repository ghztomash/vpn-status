use clap::Parser;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::path::PathBuf;

// single struct to hold configuration and arguments
#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(version, about, long_about)]
pub struct Config {
    #[arg(short, long)]
    pub no_color: bool,
    #[arg(short, long)]
    pub enabled_string: Option<String>,
    #[arg(long)]
    pub enabled_color: Option<String>,
    #[arg(long)]
    pub enabled_style: Option<String>, // emphasys
    #[arg(short, long)]
    pub disabled_string: Option<String>,
    #[arg(long)]
    pub disabled_color: Option<String>,
    #[arg(long)]
    pub disabled_style: Option<String>,
    #[arg(short, long)]
    pub config_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            no_color: false,
            enabled_string: Some("enabled".to_string()),
            enabled_color: Some("green".to_string()),
            enabled_style: None,
            disabled_string: Some("disabled".to_string()),
            disabled_color: Some("red".to_string()),
            disabled_style: None,
            config_path: None,
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

    fn parse_args() -> Self {
        Config::parse()
    }

    pub fn get() -> Self {
        let mut config = Config::default();
        // parse cli arguments
        let args = Config::parse_args();

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

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_arguments() {
        let args = vec![
            "vpn_status",
            "--enabled-string",
            "active",
            "--enabled-color",
            "blue",
            "--disabled-string",
            "inactive",
            "--disabled-color",
            "yellow",
        ];

        let config = Config::parse_from(args);
        assert_eq!(config.enabled_string, Some("active".to_string()));
        assert_eq!(config.enabled_color, Some("blue".to_string()));
        assert_eq!(config.disabled_string, Some("inactive".to_string()));
        assert_eq!(config.disabled_color, Some("yellow".to_string()));
    }

    #[test]
    fn load_config() {
        let config = Config::load_config(Some(PathBuf::from("configs/bool.toml"))).unwrap();
        assert_eq!(config.enabled_string, Some("true".to_string()));
        assert_eq!(config.disabled_string, Some("false".to_string()));
    }
}
