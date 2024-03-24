use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Struct to hold arguments
#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(version, about, long_about)]
pub struct Args {
    #[arg(short, long)]
    pub no_style: bool,
    #[arg(long)]
    pub open_config: bool,
    #[arg(short, long)]
    pub enabled_string: Option<String>,
    #[arg(long)]
    pub enabled_color: Option<String>,
    #[arg(short, long)]
    pub disabled_string: Option<String>,
    #[arg(long)]
    pub disabled_color: Option<String>,
    #[arg(short, long)]
    pub config_path: Option<PathBuf>,
    #[arg(short, long)]
    pub lookup: bool,
}

impl Args {
    /// Parse CLI arguments.
    pub fn parse_args() -> Self {
        Args::parse()
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

        let config = Args::parse_from(args);
        assert_eq!(config.enabled_string, Some("active".to_string()));
        assert_eq!(config.enabled_color, Some("blue".to_string()));
        assert_eq!(config.disabled_string, Some("inactive".to_string()));
        assert_eq!(config.disabled_color, Some("yellow".to_string()));
    }
}
