use ::voip::{voip, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = voip::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-voip operational");
    Ok(())
}
