use clap::Parser;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser, Default, Debug, Serialize, Deserialize, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub boolean: Option<bool>,
    #[arg(short, long)]
    pub no_color: Option<bool>,
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

impl Config {
    pub fn load_config() -> Result<Self> {
        let cfg: Config = confy::load("vpn_status", None)?;
        let file = confy::get_configuration_file_path("vpn_status", None)?;
        println!("The configuration file path is: {:#?}", file);
        dbg!(&cfg);
        Ok(cfg)
    }

    pub fn parse_args() -> Result<Self> {
        let args = Config::parse();
        dbg!(&args);
        Ok(args)
    }

    pub fn get() -> Self {
        let mut config = Config::default();
        if let Ok(config_file) = Config::load_config() {
            config = config_file;
        }

        // override config file with cli arguments
        if let Ok(args_config) = Config::parse_args() {
            if args_config.enabled_string.is_some() {
                config.enabled_string = args_config.enabled_string;
            }
            if args_config.enabled_color.is_some() {
                config.enabled_color = args_config.enabled_color;
            }

            if args_config.disabled_string.is_some() {
                config.disabled_string = args_config.disabled_string;
            }

            if args_config.disabled_color.is_some() {
                config.disabled_color = args_config.disabled_color;
            }

            if args_config.config_path.is_some() {
                config.config_path = args_config.config_path;
            }
        }

        config
    }
}
