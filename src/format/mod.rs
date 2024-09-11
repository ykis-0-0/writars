use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub type TarfileSpec = std::collections::HashMap<PathBuf, TarEntry>;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct TarEntry {
  content: EntryData,
  owner: Option<Identity>,
  group: Option<Identity>,
  // TODO find a way to serialize this as Octal
  mode: Option<u16>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Identity {
  ID(u32),
  Name(String),
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum EntryData {
  Directory,
  /// Hard Link (only effective within archive)
  Hardlink(PathBuf),
  /// Symbolic Link (will have target path extracted verbatim)
  Symlink(PathBuf),
  /// Character Device, specified by `(major, minor)` device number pair
  CharDev(u32, u32),
  /// Block Device, specified by `(major, minor)` device number pair
  BlockDev(u32, u32),
  /// Named Pipes for Linux
  FIFO,
  #[serde(untagged)]
  File(Option<String>),
}

mod checks;
