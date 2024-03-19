use clap::Parser;
use color_eyre::Result;
use vpn_status_lib as vpn_status;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    boolean: bool,
}

fn main() -> Result<()> {
    let cli = Args::parse();

    // println!("-------------\ndefault_net:");
    // let default_interface = netdev::get_default_interface().unwrap();
    // println!(
    //     "Interface: {:#?}  is_tun(): {}",
    //     default_interface,
    //     default_interface.is_tun()
    // );

    let status = vpn_status::get_status()?;

    if cli.boolean {
        println!("{}", vpn_status::vpn_enabled()?);
    } else {
        println!("{}", status);
    }

    Ok(())
}
