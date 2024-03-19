use clap::Parser;
use color_eyre::{owo_colors::colored, Result};
use colored::*;
use vpn_status_lib::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    boolean: bool,
    #[arg(short, long)]
    no_color: bool,
}

fn main() -> Result<()> {
    let cli = Args::parse();

    let status = get_status()?;

    let output: String = if cli.boolean {
        format!("{}", vpn_enabled()?)
    } else {
        format!("{}", status)
    };

    if cli.no_color {
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
