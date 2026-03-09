use crate::io::{writeln_redis_value_to_stdout, writeln_to_stderr};
use crate::parameters::ScanSubCommand;
use redis::{Commands, Connection, Iter, RedisResult, ScanOptions, Value};
use std::process::exit;

pub fn scan_command(connection: &mut Connection, scan: ScanSubCommand) {
    let opts = ScanOptions::default()
        .with_pattern(scan.pattern)
        .with_count(scan.count);
    let result: RedisResult<Iter<Value>> = connection.scan_options(opts);
    let output_format = String::from("{stdout}");
    let input_value = String::from("");
    match result {
        Ok(iter) => {
            for v in iter.take(scan.limit) {
                match v {
                    Ok(value) => {
                        writeln_redis_value_to_stdout(&input_value, value, &output_format)
                    },
                    Err(e) => {
                        writeln_to_stderr(e.to_string());
                        exit(1);
                    }
                }
            }
        }
        Err(e) => {
            writeln_to_stderr(e.to_string());
            exit(1);
        }
    }
}