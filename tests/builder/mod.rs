use std::fmt::Debug;

mod build;
mod collect;
mod utils;

#[derive(Debug, Default)]
pub(crate) struct MapBuilder<'a, T> {
  pub(crate) header: &'a str,
  pub(crate) raw_doc: &'a str,
  pub(crate) mod_name: &'a str,
  pub(crate) map_type: map_type::MapType,
  pub(crate) kv: &'a [(&'a str, T)],
  pub(crate) str_kv: &'a [(&'a str, &'a str)],
}

pub(crate) mod map_type {
  #[derive(Debug, Copy, Clone)]
  #[allow(dead_code)]
  pub(crate) enum MapType {
    Ordered,
    Normal,
    TinyID,
  }

  impl Default for MapType {
    fn default() -> Self {
      Self::Ordered
    }
  }

  impl MapType {
    pub(crate) const fn as_str(&self) -> &str {
      match self {
        Self::Ordered => "PhfOrderedMap",
        Self::Normal => "PhfMap",
        Self::TinyID => "PhfTinyidMap",
      }
    }
  }

  impl AsRef<str> for MapType {
    fn as_ref(&self) -> &str {
      self.as_str()
    }
  }
}
