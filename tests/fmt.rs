use std::{io, process::Command};

use itertools::Itertools;

#[test]
#[ignore]
fn fmt() -> io::Result<()> {
  let cmd = "cargo +nightly fmt"
    .split_ascii_whitespace()
    .collect_vec();

  Command::new(cmd[0])
    .args(&cmd[1..])
    .status()?;

  Ok(())
}
