use std::process::exit;
use redis::{Commands, Connection, Iter, RedisResult, ScanOptions};
use crate::io::{writeln_to_stderr, writeln_to_stdout};
use crate::parameters::ScanSubCommand;

pub fn scan_command(connection: &mut Connection, scan: ScanSubCommand) {
    let opts = ScanOptions::default()
        .with_pattern(scan.pattern)
        .with_count(scan.count);
    let result: RedisResult<Iter<String>> = connection.scan_options(opts);
    match result {
        Ok(iter) => {
            for v in iter.take(scan.limit) {
                writeln_to_stdout(v.unwrap());
            }
        }
        Err(e) => {
            writeln_to_stderr(e.to_string());
            exit(1);
        }
    }
}