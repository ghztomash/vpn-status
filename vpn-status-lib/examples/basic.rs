use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let status = vpn_status_lib::status()?;
    println!("VPN status: {status}");
    Ok(())
}
