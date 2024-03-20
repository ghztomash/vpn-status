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

use std::fmt::Display;
use thiserror::Error;

/// Error type for VPN status
#[derive(Error, Debug)]
pub enum VpnStatusError {
    #[error("Failed getting default interface")]
    DefaultInterface(String),
}

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
