use std::io;

use tap::Pipe;

mod utils;
use crate::utils::{BoolExt, run_os_cmd};

#[test]
#[ignore]
fn fmt() -> io::Result<()> {
  let err = || "Failed to run `cargo fmt` command".pipe(io::Error::other);

  r#"
    cargo +nightly fmt
  "#
  .pipe(run_os_cmd)?
  .success()
  .ok_or_else(err)
}
