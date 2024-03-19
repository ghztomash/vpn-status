use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed getting default interface")]
    DefaultInterface(String),
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Disabled,
    Enabled,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Disabled => write!(f, "disabled"),
            Self::Enabled => write!(f, "enabled"),
        }
    }
}

pub fn get_status() -> Result<Status, Error> {
    match netdev::get_default_interface() {
        Ok(interface) => {
            if interface.is_tun() {
                Ok(Status::Enabled)
            } else {
                Ok(Status::Disabled)
            }
        }
        Err(error) => Err(Error::DefaultInterface(error)),
    }
}

pub fn vpn_enabled() -> Result<bool, Error> {
    if get_status()? == Status::Enabled {
        return Ok(true);
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let result = get_status();
        assert!(result.is_ok());
    }

    #[test]
    fn test_enabled() {
        let result = vpn_enabled();
        assert!(result.is_ok());
    }
}
