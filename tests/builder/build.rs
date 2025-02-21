use core::fmt::Debug;
use std::io::{self, Write};

use tap::Pipe;

use super::{
  MapBuilder,
  utils::{create_file, init_doc_comment},
};

impl<T: Debug> MapBuilder<'_, T> {
  pub(crate) fn build(self) -> io::Result<()> {
    let doc = self
      .raw_doc
      .pipe(init_doc_comment);

    let mut file = self
      .mod_name
      .pipe(create_file)?;

    let map_type = self
      .map_type
      .unwrap_or("PhfMap");

    let collect_map_string = || match map_type {
      "PhfTinyidMap" => self.collect_tinyid(),
      "PhfOrderedMap" => self.collect_ordered_map(),
      // "PhfLangidMap" => self.collect_langid(),
      _ => self.collect_map(),
    };

    for content in [
      self.header.as_bytes(),
      doc.as_bytes(),
      b"pub const fn map<'m>() -> super::",
      map_type.as_bytes(),
      b"<'m> {\n  ",
      collect_map_string().as_bytes(),
      b"\n}",
    ] {
      file.write_all(content)?
    }

    Ok(())
  }
}
