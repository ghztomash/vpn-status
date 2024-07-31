use serde::{Deserialize, Serialize};
use std::default::Default;

/// Struct to hold configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// value to display when VPN is enabled
    pub enabled_string: Option<String>,
    /// style configuration for enabled_string
    pub enabled_style: Option<StyleConfig>,
    /// value to display when VPN is disabled
    pub disabled_string: Option<String>,
    /// style configuration for disabled_string
    pub disabled_style: Option<StyleConfig>,
    /// value to display when network is offline
    pub offline_string: Option<String>,
    /// output format
    pub output_format: Option<String>,
    /// style configuration for output_format
    pub output_style: Option<StyleConfig>,
    /// enable lookup functionality
    pub lookup: Option<bool>,
    /// list of lookup providers
    pub lookup_providers: Option<Vec<String>>,
    /// style configuration for lookup values
    pub lookup_style: Option<StyleConfig>,
}

/// Output style configuration
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
            offline_string: Some("offline".to_string()),
            output_format: None,
            output_style: None,
            lookup: Some(false),
            lookup_providers: None,
            lookup_style: None,
        }
    }
}
