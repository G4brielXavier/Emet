
mod cli;
mod core;

use emet::Emet;

use clap::Parser;
use cli::args::Args;

use cli::matches::matches;
use core::Files;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let cli = Args::parse();

    let mut files = Files::new()?;
    files.setup()?;

    let mut emet = Emet::up("".to_string());

    matches(&cli.command,&mut emet, &mut files)?;

    Ok(())

}
