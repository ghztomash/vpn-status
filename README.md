![cargo build](https://github.com/ghztomash/vpn-status/actions/workflows/rust.yml/badge.svg)
![cargo clippy](https://github.com/ghztomash/vpn-status/actions/workflows/rust-clippy.yml/badge.svg)

# VPN Status

This CLI application checks the status of a VPN connection.

It is highly customizable and is intended to be used in combination with tools like `zsh` and `tmux` as status indicator.
## Usage

By default, the application will print the status of the VPN connection to the standard output in color.
```sh
vpn-status
```

## Installation

Install the application with `cargo`:
```sh 
cargo install vpn-status
```

Alternatively, you can build the application from source:
```sh
git clone https://github.com/ghztomash/vpn-status.git
cd vpn-status
cargo build --release
cp ./target/release/vpn-status /usr/local/bin
```

## Configuration

The default configuration is stored in `./.config/vpn_status/config.toml`

Load a specified configuration with `--config-path <path>` flag, If the configuration file is not found, a copy of the default configuration will be saved in that path.

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
