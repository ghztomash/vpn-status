//! # VPN Status
//!
//! A simple library to determine whether VPN is enabled.
//! This is performed by checking if the default interface is a tun device.
//!
//! ## Usage
//! ```toml
//! [dependencies]
//! vpn-status-lib = { version = "0.1" }
//! ```
//! ## Example
//! ```rust
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let status = vpn_status_lib::status()?;
//!     println!("VPN status: {status}");
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod error;
pub mod parser;
pub mod styles;

use config::Config;
use error::VpnStatusError;
use public_ip_address::lookup::LookupProvider;
use std::fmt::Display;

/// VPN configuration status
#[derive(Debug, PartialEq)]
pub enum VpnStatus {
    Disabled,
    Enabled,
}

impl Display for VpnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Disabled => write!(f, "disabled"),
            Self::Enabled => write!(f, "enabled"),
        }
    }
}

/// Get the status of the VPN configuration.
///
/// # Example
/// ```rust
/// # use std::error::Error;
/// # use vpn_status_lib::VpnStatus;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let status = vpn_status_lib::status()?;
/// if status == VpnStatus::Enabled {
///     println!("VPN is enabled");
/// }
/// # Ok(())
/// # }
/// ```
pub fn status() -> Result<VpnStatus, VpnStatusError> {
    match netdev::get_default_interface() {
        Ok(interface) => {
            if interface.is_tun() {
                Ok(VpnStatus::Enabled)
            } else {
                Ok(VpnStatus::Disabled)
            }
        }
        Err(error) => Err(VpnStatusError::DefaultInterface(error)),
    }
}

/// Get the status of the VPN connection.
///
/// # Example
/// ```rust
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
/// if vpn_status_lib::vpn_enabled()? {
///     println!("VPN is enabled");
/// }
/// # Ok(())
/// # }
/// ```
pub fn vpn_enabled() -> Result<bool, VpnStatusError> {
    if status()? == VpnStatus::Enabled {
        return Ok(true);
    }
    Ok(false)
}

pub fn status_string(config: Config, no_style: bool) -> Result<String, VpnStatusError> {
    // TODO: remove
    #[cfg(debug_assertions)]
    dbg!(&config);

    let status = self::status()?;

    // get the custom status string if it exists
    let mut status_string: String = {
        let custom_status: Option<String> = match status {
            VpnStatus::Enabled => config.clone().enabled_string,
            VpnStatus::Disabled => config.clone().disabled_string,
        };
        custom_status.unwrap_or(format!("{}", status))
    };

    if !no_style {
        // get the custom color if it exists
        let custom_color = match status {
            VpnStatus::Enabled => {
                if let Some(ref style) = config.enabled_style {
                    style.color.clone()
                } else {
                    "".to_string()
                }
            }
            VpnStatus::Disabled => {
                if let Some(ref style) = config.disabled_style {
                    style.color.clone()
                } else {
                    "".to_string()
                }
            }
        };

        // get the custom style if it exists
        let custom_style = match status {
            VpnStatus::Enabled => {
                if let Some(style) = config.enabled_style.clone() {
                    if let Some(format) = style.format {
                        format
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            VpnStatus::Disabled => {
                if let Some(style) = config.disabled_style.clone() {
                    if let Some(format) = style.format {
                        format
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
        };

        // apply the styles to the output
        status_string = styles::apply_style(status_string, custom_style, &custom_color);
    }

    // lookup the public ip address if the flag is set
    let lookup = if config.lookup.unwrap_or(false) {
        // get custom lookup color
        let lookup_color = if let Some(ref style) = config.lookup_style {
            style.color.clone()
        } else {
            "".to_string()
        };

        // get custom lookup style
        let lookup_style = if let Some(style) = config.lookup_style {
            if let Some(format) = style.format {
                format
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        // get custom providers list
        let providers = if let Some(providers) = config.lookup_providers {
            providers
                .into_iter()
                .map(|p| p.parse::<LookupProvider>().unwrap())
                .collect()
        } else {
            vec![]
        };

        let response = if providers.is_empty() {
            public_ip_address::perform_lookup()?
        } else {
            public_ip_address::perform_cached_lookup_with(providers, Some(2))?
        };

        Some(parser::Lookup {
            ip: styles::apply_style(response.ip, lookup_style.clone(), &lookup_color),
            city: styles::apply_style(
                response.city.unwrap_or("".to_string()),
                lookup_style.clone(),
                &lookup_color,
            ),
            country: styles::apply_style(
                response.country_code.unwrap_or("".to_string()),
                lookup_style.clone(),
                &lookup_color,
            ),
        })
    } else {
        None
    };

    // get custom output format if it exists
    let format = match config.output_format {
        Some(format) => format,
        None => {
            if lookup.is_some() {
                "{status} - {city}, {country}".to_string()
            } else {
                "{status}".to_string()
            }
        }
    };

    let output = if no_style {
        parser::make_output(parser::parse(&format), &status_string, lookup)
    } else {
        // get custom color
        let color = if let Some(ref style) = config.output_style {
            style.color.clone()
        } else {
            "".to_string()
        };

        // get custom style
        let style = if let Some(style) = config.output_style {
            if let Some(format) = style.format {
                format
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        parser::make_output_styled(
            parser::parse(&format),
            &status_string,
            lookup,
            style,
            &color,
        )
    };
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let result = status();
        assert!(result.is_ok(), "Failed to get status: {:?}", result);
    }

    #[test]
    fn test_enabled() {
        let result = vpn_enabled();
        assert!(result.is_ok(), "Failed to check vpn_enabled: {:?}", result);
    }
}
