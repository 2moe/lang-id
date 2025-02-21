use std::{
  fs::File,
  io::{self, BufWriter},
};

use tap::{Pipe, Tap};

pub(crate) fn init_doc_comment(raw_doc: &str) -> String {
  let capacity = 4 + raw_doc.len() + raw_doc.lines().count() * 4;

  raw_doc
    .trim()
    .lines()
    .flat_map(|line| ["/// ", line, "\n"])
    .fold(
      String::with_capacity(capacity), //
      |acc, item| acc.tap_mut(|x| x.push_str(item)),
    )
}

pub(crate) fn create_file(mod_name: &str) -> io::Result<BufWriter<File>> {
  format!("src/maps/{mod_name}.rs")
    .tap(|x| eprintln!("pub mod {x};"))
    .tap(|x| eprintln!("rustfmt +nightly {x}"))
    .pipe_ref(File::create)?
    .pipe(BufWriter::new)
    .pipe(Ok)
}
