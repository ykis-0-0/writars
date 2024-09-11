use std::path::PathBuf;

use clap::Parser;

mod io;
mod serde;

pub(crate) use io::{find_reader, find_writer};
pub(crate) use serde::get_ron_spec;

#[derive(Debug, Parser)]
pub(crate) struct Cli {

  /// Descriptor file path, use "-" for /dev/stdin
  #[arg(long, short)]
  pub input: PathBuf,

  /// Archive output path, use "-" for /dev/stdout
  #[arg(long, short)]
  pub output: PathBuf,

  // Overwrite existing path
  #[arg(long = "force", short = 'f')]
  pub overwrite_out_path: bool

}

mod checks;
