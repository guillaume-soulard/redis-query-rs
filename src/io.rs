use std::io::{stderr, stdout, Write};
use std::process::exit;

pub fn writeln_to_stdout(str: String) {
    stdout().write(format!("{}\n", str).as_bytes()).unwrap();
}

pub fn writeln_to_stderr_and_exit(str: String) {
    stderr().write(format!("{}\n", str).as_bytes()).unwrap();
    exit(1);
}
