use std::{
  io,
  process::{Command, ExitStatus},
};

use tap::Pipe;

#[test]
#[ignore]
fn fmt() -> io::Result<()> {
  r#"
    cargo +nightly fmt
  "#
  .pipe(run_os_cmd)
  .map(|e| {
    if !e.success() {
      panic!("Failed to run os command, {e}")
    }
  })
}

fn run_os_cmd(raw: &str) -> io::Result<ExitStatus> {
  raw
    .trim_ascii()
    .pipe(shlex::Shlex::new)
    .pipe_ref_mut(|it| {
      it.next()
        .expect("Invalid command")
        .pipe(Command::new)
        .args(it)
        .status()
    })
}
