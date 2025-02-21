use std::{
  io,
  process::{Command, ExitStatus},
};

use tap::{Pipe, Tap};
type ExitResult = io::Result<ExitStatus>;

#[ignore]
#[test]
fn build_and_open_rust_doc() -> io::Result<()> {
  if !build_rsdoc()?.success() {
    panic!("Failed to build rust doc")
  }
  open_doc_html()?;

  Ok(())
}

fn build_rsdoc() -> ExitResult {
  let err = || "Invalid cargo rustdoc command".pipe(io::Error::other);

  r#"
    cargo +nightly
    rustdoc
    --all-features
    --
    --cfg  __unstable_doc
    --document-private-items
  "#
  .pipe(shlex::split)
  .ok_or_else(err)?
  .tap(|x| eprintln!("{x:?}"))
  .pipe_deref(run_os_cmd)
}

fn run_os_cmd(cmd: &[String]) -> ExitResult {
  Command::new(&cmd[0])
    .args(&cmd[1..])
    .status()
}

fn open_doc_html() -> ExitResult {
  let html = format!(
    "{dir}/doc/{pkg}/index.html",
    dir = match env!("CARGO_TARGET_DIR") {
      "" => concat!(env!("CARGO_MANIFEST_DIR"), "/target"),
      s => s,
    },
    pkg = env!("CARGO_PKG_NAME").replace("-", "_")
  );

  Command::new("open")
    .arg(html)
    .status()
}
