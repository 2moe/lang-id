//! ```ignore, sh
//! cargo open-doc
//! ```
use std::{
  io,
  process::{Command, ExitStatus},
};

use tap::{Pipe, Tap};
type ExitResult = io::Result<ExitStatus>;

#[ignore]
#[test]
// #[allow(clippy::single_element_loop)]
fn build_and_open_rust_doc() -> io::Result<()> {
  let pkg = env!("CARGO_PKG_NAME");
  // for pkg in [
  //   // "hlight-dump", //
  // ] {
  if !build_rsdoc(pkg)?.success() {
    panic!("Failed to build rust doc")
  }
  // }

  Ok(())
}

fn build_rsdoc(pkg: &str) -> ExitResult {
  let err = || "Invalid cargo rustdoc command".pipe(io::Error::other);

  format!(
    "
    cargo +nightly
    rustdoc
    -p {pkg}
    --all-features
    --open
    --
    --cfg  __unstable_doc
    --document-private-items
  "
  )
  .trim_ascii()
  .lines()
  .filter(|x| !x.trim().starts_with("//"))
  .collect::<String>()
  .pipe_deref(shlex::split)
  .ok_or_else(err)?
  .tap(|x| eprintln!("{x:?}"))
  .pipe_deref(run_os_cmd)
}

fn run_os_cmd(cmd: &[String]) -> ExitResult {
  Command::new(&cmd[0])
    .args(&cmd[1..])
    .status()
}
