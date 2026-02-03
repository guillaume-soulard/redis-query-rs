extern crate core;
mod parameters;
mod connection;
mod env;
mod io;

use std::process::exit;
use crate::connection::connect;
use crate::env::{describe_env, list_env, load_env_parameters, remove_env, set_env};
use crate::io::{writeln_to_stderr_and_exit, writeln_to_stdout};
use crate::parameters::{load_parameters, EnvSubCommand, ExecSubCommand, RqParameters, RqSubCommand, ScanSubCommand};
use redis::{Connection, Iter, RedisResult, ScanOptions, TypedCommands};

fn main() {
    let parameters: RqParameters = load_parameters();
    match parameters.command {
        RqSubCommand::Exec(mut cmd) => {
            load_env_parameters(&mut cmd);
            let mut con = connect(&cmd);
            exec_command(&mut con, cmd);
        }
        RqSubCommand::Scan(mut scan) => {
            load_env_parameters(&mut scan);
            let mut con = connect(&scan);
            scan_command(&mut con, scan);
        },
        RqSubCommand::Env(env) => {
            match env {
                EnvSubCommand::Set(set_env_cmd) => set_env(set_env_cmd),
                EnvSubCommand::List(_) => list_env(),
                EnvSubCommand::Remove(remove_env_cmd) => remove_env(remove_env_cmd),
                EnvSubCommand::Describe(describe_env_cmd) => describe_env(describe_env_cmd),
            }
        }
    }
}

fn scan_command(connection: &mut Connection, scan: ScanSubCommand) {
    let opts = ScanOptions::default()
        .with_pattern(scan.pattern)
        .with_count(scan.count);
    let result: RedisResult<Iter<String>> = connection.scan_options(opts);
    match result {
        Ok(iter) => {
            let mut counter: usize = 0;
            for v in iter {
                writeln_to_stdout(v.unwrap());
                counter += 1;
                if counter >= scan.limit {
                    break;
                }
            }
        }
        Err(e) => {
            writeln_to_stderr_and_exit(e.to_string());
            exit(1);
        }
    }
}

fn exec_command(connection: &mut Connection, exec_command: ExecSubCommand) {
    let commands = exec_command.command.split(' ').collect::<Vec<&str>>();
    execute(connection, commands);
}

fn execute(con: &mut Connection, command: Vec<&str>) {
    if command.len() == 0 {
        return;
    }
    let name = command[0];
    let mut cmd = &mut redis::cmd(name);
    for &c in command.iter().skip(1) {
        cmd = cmd.arg(c)
    }
    match cmd.query::<String>(con) {
        Ok(r) => writeln_to_stdout(format!("{}", r)),
        Err(e) => writeln_to_stderr_and_exit(e.to_string()),
    }
}
