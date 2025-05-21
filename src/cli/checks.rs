#![cfg(test)]
use eyre::Result as eResult;
use crate::lib;

const THE_SPECIMEN: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/mocks/extensive.ron"));
const THE_SANITY_CHECK: &'static str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/mocks/minimal.ron"));

#[test]
fn test_sample_deserialize() -> eResult<()> {
  let parse_result = super::get_ron_spec().from_str::<lib::TarfileSpec>(THE_SPECIMEN)?;

  println!("{:#?}", parse_result);

  Ok(())
}

#[test]
fn test_sanity() -> eResult<()> {
  let parse_result = super::get_ron_spec().from_str::<lib::TarfileSpec>(THE_SANITY_CHECK)?;

  println!("{:#?}", parse_result);

  Ok(())
}