use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VpnStatusError {
    #[error("Failed getting default interface")]
    DefaultInterface(String),
}

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
        assert!(result.is_ok());
    }

    #[test]
    fn test_enabled() {
        let result = vpn_enabled();
        assert!(result.is_ok());
    }
}
