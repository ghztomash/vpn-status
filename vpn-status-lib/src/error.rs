use thiserror::Error;

/// Error type for VPN status
#[derive(Error, Debug)]
pub enum VpnStatusError {
    #[error("Failed getting default interface")]
    DefaultInterface(String),
}
