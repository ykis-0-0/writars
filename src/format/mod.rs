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
  Pair(String, u32)
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub enum EntryData {
  /// File, None for a 0-byte one
  File(#[serde(with = "serde_bytes")] Option<Vec<u8>>),
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

type FullIdentity = (Option<String>, Option<u32>);

impl From<Identity> for FullIdentity {
  fn from(value: Identity) -> Self {
    match value {
        Identity::ID(val) => (None, Some(val)),
        Identity::Name(val) => (Some(val), None),
        Identity::Pair(name, id) => (Some(name), Some(id)),
    }
  }
}

impl TryFrom<TarEntry> for (tar::Header, Vec<u8>) {
  type Error = std::io::Error;

  fn try_from(row: TarEntry) -> ioResult<(tar::Header, Vec<u8>)> {
    let mut header = tar::Header::new_ustar();

    if let Some((maybe_owner, maybe_uid)) = row.owner.map(Into::<FullIdentity>::into) {
      if let Some(one) = maybe_owner {
        header.set_username(&one)?;
      }
      if let Some(id) = maybe_uid {
        header.set_uid(id.into());
      }
    }

    if let Some((maybe_group, maybe_gid)) = row.group.map(Into::<(Option<String>, Option<u32>)>::into) {
      if let Some(group) = maybe_group {
        header.set_groupname(&group)?;
      }
      if let Some(id) = maybe_gid {
        header.set_gid(id.into());
      }
    }

    // Type handlings and default mode
    header.set_entry_type((&row.content).into());
    let default_m_mode = match row.content {
      EntryData::File(_) => {
        0o0664
      },
      EntryData::Directory => {
        0o0775
      },
      EntryData::Hardlink(ref p) => {
        header.set_link_name(p)?;
        0o0664
      },
      EntryData::Symlink(ref p) => {
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

    let content = match row.content {
        EntryData::File(Some(bytes)) => bytes,
        _ => Vec::new(),
    };

    Ok((header, content))
  }
}

pub fn mk_archive<P: AsRef<Path>>(from: std::collections::HashMap<P, TarEntry>) -> std::io::Result<Vec<u8>> {

  fn decoupler<P>((path, data): (P, TarEntry)) -> ioResult<(P, tar::Header, Box<dyn std::io::Read>, u64)> {
    use std::collections::VecDeque;
    let (header, content) = data.try_into()?;

    let (size, reader) = (content.len() as u64, Box::new(VecDeque::<_>::from(content)));

    Ok((path, header, reader, size))
  }

  let rows = from.into_iter()
    .map(decoupler)
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
