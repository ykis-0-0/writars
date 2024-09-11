use eyre::{Result as eResult, OptionExt};

pub fn find_reader(path: &std::path::Path) -> eResult<Box<dyn std::io::Read>> {
  use std::io::stdin;

  let unwrapped = path.to_str()
  .ok_or_eyre("Unable to parse path")?;

  if unwrapped == "-" {
    return Ok(Box::new(stdin()));
  }

  let file = std::fs::OpenOptions::new()
  .read(true).write(false).append(false)
  .create(false).truncate(false)
  .open(path)?;

  Ok(Box::new(file))
}

pub fn find_writer(path: &std::path::Path, overwrite_out: bool) -> eResult<Box<dyn std::io::Write>> {
  use std::io::stdout;

  let unwrapped = path.to_str()
  .ok_or_eyre("Unable to parse path")?;

  if unwrapped == "-" {
    return Ok(Box::new(stdout()));
  }

  let file = std::fs::OpenOptions::new()
  .read(false).write(true).append(false)
  .create_new(!overwrite_out).truncate(overwrite_out)
  .open(path)?;

  Ok(Box::new(file))
}

