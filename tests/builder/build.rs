use core::fmt::Debug;
use std::io::{self, Write};

use tap::Pipe;

use super::{
  MapBuilder,
  utils::{create_file, init_doc_comment},
};
use crate::builder::map_type::MapType;

impl<T: Debug> MapBuilder<'_, T> {
  pub(crate) fn build(self) -> io::Result<()> {
    let doc = self
      .raw_doc
      .pipe(init_doc_comment);

    let mut file = self
      .mod_name
      .pipe(create_file)?;
    let map_type = self.map_type;

    let collect_map_string = || match map_type {
      MapType::Normal => self.collect_map(),
      MapType::TinyID => self.collect_tinyid(),
      MapType::Ordered => self.collect_ordered_map(),
      // "PhfTinyidMap" => self.collect_tinyid(),
      // "PhfOrderedMap" => self.collect_ordered_map(),
      // "PhfLangidMap" => self.collect_langid(),
      // "PhfMap"
      // _ => self.collect_map(),
    };

    for content in [
      self.header.as_bytes(),
      doc.as_bytes(),
      b"pub const fn map<'m>() -> super::",
      map_type.as_str().as_bytes(),
      b"<'m> {\n  ",
      collect_map_string().as_bytes(),
      b"\n}",
    ] {
      file.write_all(content)?
    }

    Ok(())
  }
}
