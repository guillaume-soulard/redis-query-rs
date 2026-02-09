use std::io::{stderr, stdout, Write};

pub fn writeln_to_stdout(str: String) {
    stdout().write(format!("{}\n", str).as_bytes()).unwrap();
}

pub fn writeln_to_stderr(str: String) {
    stderr().write(format!("{}\n", str).as_bytes()).unwrap();
}
