//! ```ignore, sh
//! cargo open-doc
//! ```
use std::{fmt::Display, io, process::ExitStatus};

use tap::Pipe;
mod utils;
use crate::utils::run_os_cmd;

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  let err = || "Failed to run `cargo rustdoc` command".pipe(io::Error::other);

  env!("CARGO_PKG_NAME")
    .pipe(build_rsdoc)?
    .success()
    .then_some(())
    .ok_or_else(err)
}
fn build_rsdoc<T: Display>(pkg: T) -> io::Result<ExitStatus> {
  format!(
    r#"
    cargo +nightly
    rustdoc
    -p  {pkg}
    --all-features
    --open
    --
    --cfg  __unstable_doc
    --document-private-items
  "#
  )
  .pipe_deref(run_os_cmd)
}
