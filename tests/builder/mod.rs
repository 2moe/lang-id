use std::fmt::Debug;

mod build;
mod collect;
mod utils;

#[derive(Debug, Default)]
pub(crate) struct MapBuilder<'a, T> {
  pub(crate) header: &'a str,
  pub(crate) raw_doc: &'a str,
  pub(crate) mod_name: &'a str,
  pub(crate) map_type: Option<&'a str>,
  pub(crate) kv: &'a [(&'a str, T)],
  pub(crate) str_kv: &'a [(&'a str, &'a str)],
}
