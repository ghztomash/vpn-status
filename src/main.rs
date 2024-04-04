use args::Args;
use color_eyre::Result;
use config::Config;
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
                    "".to_string()
                }
            }
            VpnStatus::Disabled => {
                if let Some(ref style) = config.disabled_style {
                    style.color.clone()
                } else {
                    "".to_string()
                }
            }
        };

        // get the custom style if it exists
        let custom_style = match status {
            VpnStatus::Enabled => {
                if let Some(style) = config.enabled_style.clone() {
                    if let Some(format) = style.format {
                        format
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
            VpnStatus::Disabled => {
                if let Some(style) = config.disabled_style.clone() {
                    if let Some(format) = style.format {
                        format
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            }
        };

        // apply the styles to the output
        status_string = styles::apply_style(status_string, custom_style, &custom_color);
    }

    let output;

    // lookup the public ip address if the flag is set
    let lookup = if config.lookup.unwrap_or(false) {
        // get custom lookup color
        let lookup_color = if let Some(ref style) = config.lookup_style {
            style.color.clone()
        } else {
            "".to_string()
        };

        // get custom lookup style
        let lookup_style = if let Some(style) = config.lookup_style {
            if let Some(format) = style.format {
                format
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let response = public_ip_address::perform_lookup()?;
        Some(parser::Lookup {
            ip: styles::apply_style(response.ip, lookup_style.clone(), &lookup_color),
            city: styles::apply_style(
                response.city.unwrap_or("".to_string()),
                lookup_style.clone(),
                &lookup_color,
            ),
            country: styles::apply_style(
                response.country_code.unwrap_or("".to_string()),
                lookup_style.clone(),
                &lookup_color,
            ),
        })
    } else {
        None
    };

    // get custom output format if it exists
    let format = match config.output_format {
        Some(format) => format,
        None => {
            if lookup.is_some() {
                "{status} - {city}, {country}".to_string()
            } else {
                "{status}".to_string()
            }
        }
    };

    if args.no_style {
        output = parser::make_output(parser::parse(&format), &status_string, lookup);
    } else {
        // get custom color
        let color = if let Some(ref style) = config.output_style {
            style.color.clone()
        } else {
            "".to_string()
        };

        // get custom style
        let style = if let Some(style) = config.output_style {
            if let Some(format) = style.format {
                format
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        output = parser::make_output_styled(
            parser::parse(&format),
            &status_string,
            lookup,
            style,
            &color,
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
