pub fn get_ron_spec() -> ron::Options {
  use ron::{Options, extensions::Extensions};

  macro_rules! extlist {
      ( $( $e:ident )* ) => { $(Extensions::$e) |* };
  }

  let exts = extlist!{
    UNWRAP_NEWTYPES
    IMPLICIT_SOME
    UNWRAP_VARIANT_NEWTYPES
  };

  Options::default()
  .with_default_extension(exts)
}
