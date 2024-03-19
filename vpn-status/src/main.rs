use color_eyre::Result;
use colored::*;
use vpn_status_lib::*;

mod config;
use config::Config;

fn main() -> Result<()> {
    // load the config from file or args
    let config = Config::get();
    let status = get_status()?;

    // get the custom status string if it exists
    let output: String = {
        let custom_status: Option<String> = match status {
            Status::Enabled => config.clone().enabled_string,
            Status::Disabled => config.clone().disabled_string,
        };
        custom_status.unwrap_or(format!("{}", status))
    };

    if config.no_color {
        println!("{}", output);
    } else {
        // get the custom color if it exists
        let custom_color = match status {
            Status::Enabled => config.clone().enabled_color.unwrap_or("green".to_string()),
            Status::Disabled => config.clone().disabled_color.unwrap_or("red".to_string()),
        };
        let color = colored::Color::from(custom_color);
        println!("{}", output.color(color));
    }
    Ok(())
}
