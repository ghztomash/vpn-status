use color_eyre::Result;
use colored::*;
use config::Config;
use vpn_status_lib::VpnStatus;

mod config;
mod styles;

fn main() -> Result<()> {
    // install color_eyre error handling
    color_eyre::install()?;

    // load the config from file or args
    let config = Config::get();
    let status = vpn_status_lib::status()?;

    // TODO: remove
    #[cfg(debug_assertions)]
    dbg!(&config);

    // get the custom status string if it exists
    let output: String = {
        let custom_status: Option<String> = match status {
            VpnStatus::Enabled => config.clone().enabled_string,
            VpnStatus::Disabled => config.clone().disabled_string,
        };
        custom_status.unwrap_or(format!("{}", status))
    };

    if config.no_color {
        println!("{}", output);
    } else {
        // get the custom color if it exists
        let custom_color = match status {
            VpnStatus::Enabled => config.clone().enabled_color.unwrap_or("green".to_string()),
            VpnStatus::Disabled => config.clone().disabled_color.unwrap_or("red".to_string()),
        };
        let color = colored::Color::from(custom_color);

        // get the custom style if it exists
        let custom_style = match status {
            VpnStatus::Enabled => config.clone().enabled_style.unwrap_or("clear".to_string()),
            VpnStatus::Disabled => config.clone().disabled_style.unwrap_or("clear".to_string()),
        };

        let style = styles::styles_from_str(&custom_style)?;
        let output = styles::style(output, style);

        // apply the styles to the output
        println!("{}", output.color(color));
    }
    Ok(())
}
