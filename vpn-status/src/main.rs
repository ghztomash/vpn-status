use color_eyre::Result;
use colored::*;
use vpn_status_lib::*;

mod config;
use config::Config;

fn main() -> Result<()> {
    // load the config from file or args
    let config = Config::get();
    let status = get_status()?;

    let output: String = {
        let custom_status: Option<String> = match status {
            Status::Enabled => config.enabled_string,
            Status::Disabled => config.disabled_string,
        };
        custom_status.unwrap_or(format!("{}", status))
    };

    if config.no_color {
        println!("{}", output);
    } else {
        let color = if status == vpn_status_lib::Status::Enabled {
            colored::Color::Green
        } else {
            colored::Color::Red
        };
        println!("{}", output.color(color));
    }

    Ok(())
}
