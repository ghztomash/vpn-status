use args::Args;
use color_eyre::Result;
use log::{debug, error};

mod args;
mod config;

fn main() -> Result<()> {
    // install color_eyre error handling
    color_eyre::install()?;
    // install logger
    env_logger::init();

    // load the config from file or args
    let args = Args::parse_args();

    if args.open_config {
        return open_config();
    }

    debug!("tunnel_name: {:?}", vpn_status_lib::tunnel_name());
    debug!("tunnel_address: {:?}", vpn_status_lib::tunnel_address());

    let config = config::get(args.clone());
    let output = vpn_status_lib::status_string(config, args.no_style)?;

    print!("{}", output);
    Ok(())
}

/// Open the default configuration file in the default editor
fn open_config() -> Result<()> {
    let config_path = confy::get_configuration_file_path("vpn_status", Some("config"))?;
    println!("Opening default configuration file: {:?}", config_path);
    let _ = std::process::Command::new("open")
        .arg(config_path)
        .output()?;
    Ok(())
}
