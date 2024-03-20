# VPN Status Library
This crate provides a library for checking the status of a VPN connection across multiple platforms.
This is performed by checking if the default interface is a `tun` device.

For the stand-lone CLI application, see the [vpn-status](../) crate.

## Usage
```toml
[dependencies]
vpn-status-lib = { version = "0.1" }
```
## Example
```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let status = vpn_status_lib::status()?;
    println!("VPN status: {status}");
    Ok(())
}
```
