#![cfg(test)]
use eyre::Result as eResult;
use crate::lib;

const THE_SPECIMEN: &'static str = include_str!("./specimen/extensive.ron");

#[test]
fn test_sample_deserialize() -> eResult<()>{
  let parse_result = super::get_ron_spec().from_str::<lib::TarfileSpec>(THE_SPECIMEN)?;

  println!("{:?}", parse_result);

  Ok(())
}
