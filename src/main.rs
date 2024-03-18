use color_eyre::Result;

fn main() -> Result<()> {
    println!("-------------\ndefault_net:");
    let default_interface = netdev::get_default_interface().unwrap();
    println!(
        "Interface: {:#?}  is_tun(): {}",
        default_interface,
        default_interface.is_tun()
    );

    let status = vpn_status::get_status()?;

    println!("vpn status: {}", status);

    Ok(())
}
