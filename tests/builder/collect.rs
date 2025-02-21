use core::fmt::Debug;

use tap::Pipe;

use super::MapBuilder;

macro_rules! new_phf_map {
  ($map:ident) => {
    phf_codegen::$map::new().phf_path("super::phf")
  };
}

macro_rules! collect_into_map {
  () => {
    |map, (key, value)| {
      format!(r#####"r###"{value}"###"#####) //
        .pipe_ref(|v| map.entry(key, v));
      map
    }
  };
}

macro_rules! collect_str_kv_map {
  ($self:ident, $map:ident) => {
    $self
      .str_kv
      .iter()
      .fold(new_phf_map!($map), collect_into_map!())
      .build()
      .to_string()
  };
}

impl<T> MapBuilder<'_, T> {
  pub(crate) fn collect_map(&self) -> String {
    collect_str_kv_map!(self, Map)
  }

  pub(crate) fn collect_ordered_map(&self) -> String {
    collect_str_kv_map!(self, OrderedMap)
  }
}

impl<T: Debug> MapBuilder<'_, T> {
  pub(crate) fn collect_tinyid(&self) -> String {
    self
      .kv
      .iter()
      .fold(new_phf_map!(Map), |map, (key, tuple)| {
        format!("const {{ TinyID::new{tuple:?} }}") //
          .pipe_ref(|v| map.entry(key, v));
        map
      })
      .build()
      .to_string()
  }
}
