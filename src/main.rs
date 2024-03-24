use args::Args;
use color_eyre::Result;
use colored::*;
use config::Config;
use public_ip_address::lookup::LookupProvider;
use vpn_status_lib::VpnStatus;

mod args;
mod config;
mod parser;
mod styles;

fn main() -> Result<()> {
    // install color_eyre error handling
    color_eyre::install()?;

    // load the config from file or args
    let args = Args::parse_args();

    if args.open_config {
        return open_config();
    }

    let config = Config::get(args.clone());
    let status = vpn_status_lib::status()?;

    // TODO: remove
    #[cfg(debug_assertions)]
    dbg!(&config);

    // get the custom status string if it exists
    let mut status_string: String = {
        let custom_status: Option<String> = match status {
            VpnStatus::Enabled => config.clone().enabled_string,
            VpnStatus::Disabled => config.clone().disabled_string,
        };
        custom_status.unwrap_or(format!("{}", status))
    };

    if !args.no_style {
        // get the custom color if it exists
        let custom_color = match status {
            VpnStatus::Enabled => {
                if let Some(ref style) = config.enabled_style {
                    style.color.clone()
                } else {
                    "green".to_string()
                }
            }
            VpnStatus::Disabled => {
                if let Some(ref style) = config.disabled_style {
                    style.color.clone()
                } else {
                    "red".to_string()
                }
            }
        };
        let color = colored::Color::from(custom_color);

        // get the custom style if it exists
        let custom_style = match status {
            VpnStatus::Enabled => {
                if let Some(style) = config.enabled_style.clone() {
                    if let Some(format) = style.format {
                        format
                    } else {
                        vec!["clear".to_string()]
                    }
                } else {
                    vec!["clear".to_string()]
                }
            }
            VpnStatus::Disabled => {
                if let Some(style) = config.disabled_style.clone() {
                    if let Some(format) = style.format {
                        format
                    } else {
                        vec!["clear".to_string()]
                    }
                } else {
                    vec!["clear".to_string()]
                }
            }
        };

        let custom_style: Vec<&str> = custom_style.iter().map(|x| x.as_ref()).collect();
        let style = styles::styles_from_vec(custom_style)?;
        let output = styles::style(status_string, style);

        // apply the styles to the output
        status_string = format!("{}", output.color(color));
    }

    let mut output;
    // get custom output format if it exists
    if let Some(format) = config.output_format {
        output = parser::make_output(parser::parse(&format), &status_string);
    } else {
        output = status_string;
    }

    if config.lookup.unwrap_or(false) {
        let response = public_ip_address::perform_lookup_with(LookupProvider::IfConfig).unwrap();
        output = format!(
            "{} in {} {}",
            output,
            response.city.unwrap_or("".to_string()),
            response.country_code.unwrap_or("".to_string())
        );
    }

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
