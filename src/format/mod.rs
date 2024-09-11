use std::path::{Path, PathBuf};
use std::io::Result as ioResult;

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

impl From<&EntryData> for tar::EntryType {
  fn from(value: &EntryData) -> Self {
    use tar::EntryType;

    match value {
      EntryData::File(_) => EntryType::Regular,
      EntryData::Directory => EntryType::Directory,
      EntryData::Hardlink(_) => EntryType::Link,
      EntryData::Symlink(_) => EntryType::Symlink,
      EntryData::CharDev(_, _) => EntryType::Char,
      EntryData::BlockDev(_, _) => EntryType::Block,
      EntryData::FIFO => EntryType::Fifo
    }
  }
}

impl TryFrom<TarEntry> for (tar::Header, Option<String>) {
  type Error = std::io::Error;

  fn try_from(row: TarEntry) -> ioResult<(tar::Header, Option<String>)> {
    let mut header = tar::Header::new_ustar();

    let (row, content) = {
      let mut row = row;
      let content = match &mut row.content {
        EntryData::File(option) => option.take(),
        _ => None,
      };
      (row, content)
    };

    if let Some(thing) = row.owner {
      match thing {
        Identity::ID(n) => header.set_uid(n.into()),
        Identity::Name(who) => header.set_username(&who)?,
      }
    }

    if let Some(thing) = row.group {
      match thing {
        Identity::ID(n) => header.set_gid(n.into()),
        Identity::Name(which) => header.set_groupname(&which)?,
      }
    }

    // Type handlings and default mode
    header.set_entry_type((&row.content).into());
    let default_m_mode = match row.content {
      EntryData::File(Some(_)) => {
        unreachable!("Erroneous handling, string data will be lost");
      },
      EntryData::File(None) => {
        0o0664
      },
      EntryData::Directory => {
        0o0775
      },
      EntryData::Hardlink(p) => {
        header.set_link_name(p)?;
        0o0664
      },
      EntryData::Symlink(p) => {
        header.set_link_name(p)?;
        0o0777
      },
      EntryData::CharDev(maj, min) | EntryData::BlockDev(maj, min)=> {
        header.set_device_major(maj)?;
        header.set_device_minor(min)?;
        0o0777
      },
      EntryData::FIFO => {
        0o0777
      },
    };
    header.set_mode(row.mode.unwrap_or(default_m_mode).into());

    Ok((header, content))
  }
}

pub fn mk_archive<P: AsRef<Path>>(from: std::collections::HashMap<P, TarEntry>) -> std::io::Result<Vec<u8>> {
  use std::io::Read;

  fn extractor<P>((path, data): (P, TarEntry)) -> ioResult<(P, tar::Header, Box<dyn std::io::Read>, u64)> {
    use std::collections::VecDeque;
    let (header, content) = data.try_into()?;

    let (size, reader) = content
      .map(String::into_bytes)
      .map(VecDeque::from)
      .map(Box::new)
      .map(|b| (b.len() as u64, b as Box<dyn Read>))
      .unwrap_or((0, Box::new(std::io::empty())))
    ;

    Ok((path, header, reader, size))
  }

  let rows = from.into_iter()
  .map(extractor)
  .collect::<ioResult<Vec<_>>>()?;
  let mut builder = tar::Builder::new(Vec::new());

  for (path, mut header, content, size) in rows {

    header.set_size(size);

    builder.append_data(&mut header, &path, content)?;
  }

  // `.finish()`-ed when invoked
  builder.into_inner()

}

mod checks;
