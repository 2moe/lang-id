use std::{
  io,
  process::{Command, ExitStatus},
};

use tap::Pipe;

pub(crate) fn run_os_cmd(raw: &str) -> io::Result<ExitStatus> {
  raw
    .trim_ascii()
    /* */
    // .lines()
    // .filter(|x| !x.trim().starts_with("//"))
    // .collect::<String>()
    // .as_str()
    /* */
    .pipe(shlex::Shlex::new)
    .pipe_ref_mut(|it| {
      it.next()
        .expect("Invalid command")
        .pipe(Command::new)
        .args(it.inspect(|s| eprint!("{s:?} ")))
        .status()
        // .tap(|_|eprintln!())
    })
}

pub(crate) trait BoolExt {
  fn ok_or_else<E, F>(self, err: F) -> Result<(), E>
  where
    Self: Into<bool>,
    F: FnOnce() -> E,
  {
    if self.into() { Ok(()) } else { Err(err()) }
  }
}

impl BoolExt for bool {}
