use clap::Parser;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::path::PathBuf;

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub no_color: bool,
    #[arg(long)]
    pub enabled_string: Option<String>,
    #[arg(long)]
    pub enabled_color: Option<String>,
    #[arg(long)]
    pub disabled_string: Option<String>,
    #[arg(long)]
    pub disabled_color: Option<String>,
    #[arg(long)]
    pub config_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            no_color: false,
            enabled_string: Some("enabled".to_string()),
            enabled_color: Some("green".to_string()),
            disabled_string: Some("disabled".to_string()),
            disabled_color: Some("red".to_string()),
            config_path: None,
        }
    }
}

impl Config {
    pub fn load_config(path: Option<PathBuf>) -> Result<Self> {
        let config: Config = match path {
            Some(path) => confy::load_path(path)?,
            None => confy::load("vpn_status", Some("config"))?,
        };
        let file = confy::get_configuration_file_path("vpn_status", Some("config"))?;
        println!("default configuration file is: {:#?}", file);
        Ok(config)
    }

    pub fn parse_args() -> Self {
        Config::parse()
    }

    pub fn get() -> Self {
        let mut config = Config::default();
        // parse cli arguments
        let args = Config::parse_args();
        dbg!(&args);

        // load config file
        if let Ok(config_file) = Config::load_config(args.config_path) {
            config = config_file;
        }
        dbg!(&config);

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
