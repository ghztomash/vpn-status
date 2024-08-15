use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Struct to hold command line arguments
#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Do not apply any styles
    #[arg(short, long)]
    pub no_style: bool,
    /// Open the default configuration file
    #[arg(short = 'O', long)]
    pub open_config: bool,
    /// Value to display when VPN is enabled
    #[arg(short, long)]
    pub enabled_string: Option<String>,
    /// Color of enabled_string
    #[arg(long)]
    pub enabled_color: Option<String>,
    /// Value to display when VPN is disabled
    #[arg(short, long)]
    pub disabled_string: Option<String>,
    /// Color of disabled_string
    #[arg(long)]
    pub disabled_color: Option<String>,
    /// Value to display when network is offline
    #[arg(short, long)]
    pub offline_string: Option<String>,
    /// Value to display when split tunnel is set
    #[arg(short, long)]
    pub split_tunnel_string: Option<String>,
    /// Color of split_tunnel_string
    #[arg(long)]
    pub split_tunnel_color: Option<String>,
    /// Output format
    #[arg(short = 'f', long)]
    pub output_format: Option<String>,
    /// Path to configuration file
    #[arg(short, long)]
    pub config_path: Option<PathBuf>,
    /// Enable lookup functionality
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
            "--offline-string",
            "offline",
        ];

        let config = Args::parse_from(args);
        assert_eq!(config.enabled_string, Some("active".to_string()));
        assert_eq!(config.enabled_color, Some("blue".to_string()));
        assert_eq!(config.disabled_string, Some("inactive".to_string()));
        assert_eq!(config.disabled_color, Some("yellow".to_string()));
        assert_eq!(config.offline_string, Some("offline".to_string()));
    }
}
