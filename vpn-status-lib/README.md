# VPN Statuso Library
This crate provides a library for checking the status of a VPN connection across multiple platforms.
This is performed by checking if the default interface is a `tun` device.

For the stand-lone CLI application, see the [vpn-status](../) crate.

## Usage

Add the following to your `Cargo.toml` file:
```toml
[dependencies]
vpn-status-lib = { version = "0.1" }
```
## Example

The simplest way to use this library is to call the `status()` function, which returns a `Result` with a `VpnStatus`.
```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let status = vpn_status_lib::status()?;
    println!("VPN status: {status}");
    Ok(())
}
```
## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
