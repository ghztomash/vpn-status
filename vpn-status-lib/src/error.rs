use thiserror::Error;

/// Error type for VPN status
#[derive(Error, Debug)]
pub enum VpnStatusError {
    #[error("Failed getting default interface")]
    DefaultInterface(String),
    #[error("Failed styling")]
    StyleError(String),
    #[error("Failed performing lookup")]
    LookupError(#[from] public_ip_address::error::Error),
}
