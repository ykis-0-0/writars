use eyre::Result as eResult;
use clap::Parser;

use wri_ta_rs as lib;
mod cli;

fn main() -> eResult<()>{
  color_eyre::install()?;

  let args = cli::Cli::parse();

  Ok(())
}
