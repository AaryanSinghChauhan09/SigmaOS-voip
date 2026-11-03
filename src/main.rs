use anyhow::Result;
use clap::Parser;
use voip::voip;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_integration: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = voip::Config { 
        desktop_integration: !cli.no_integration, 
        ..Default::default() 
    };
    let mut component = voip::new(config);
    
    component.initialize()?;
    component.launch()?;
    
    println!("SigmaOS-voip ready");
    Ok(())
}
