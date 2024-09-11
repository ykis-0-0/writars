use eyre::{Result as eResult, Context};
use clap::Parser;

use wri_ta_rs as lib;
mod cli;

fn main() -> eResult<()>{
  color_eyre::install()?;

  let args = cli::Cli::parse();

  let fd = cli::find_reader(&args.input)
  .wrap_err(format!("When opening input file [{}]", args.input.display()))?;

  let input = std::io::read_to_string(fd)
  .wrap_err("While reading input file")?;

  let spec = cli::get_ron_spec().from_str::<lib::TarfileSpec>(&input)
  .wrap_err("When parsing input file")?;

  let bytes = lib::mk_archive(spec).wrap_err("While building result archive")?;

  cli::find_writer(&args.output, args.overwrite_out_path)
  .wrap_err(format!("When opening output file [{}]", args.output.display()))?
  .write_all(&bytes)
  .wrap_err("While writing to output")?;

  Ok(())
}
