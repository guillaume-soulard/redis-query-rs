extern crate core;
mod parameters;
mod connection;
mod env;
mod io;

use crate::connection::connect;
use crate::env::{describe_env, list_env, load_env_parameters, remove_env, set_env};
use crate::io::{writeln_to_stderr, writeln_to_stdout};
use crate::parameters::{load_parameters, EnvSubCommand, ExecSubCommand, RqParameters, RqSubCommand, ScanSubCommand};
use redis::{Connection, Iter, RedisResult, ScanOptions, TypedCommands};
use std::io::{stdin, Stdin};
use std::process::exit;

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
            writeln_to_stderr(e.to_string());
            exit(1);
        }
    }
}

const FIX_ARGUMENT_PLACE_HOLDER: &'static str = "{?}";
const ITERATOR_ARGUMENT_PLACE_HOLDER: &'static str = "{>}";

fn exec_command(connection: &mut Connection, exec_command: ExecSubCommand) {
    let commands = exec_command.command.split(' ').collect::<Vec<&str>>();
    let has_std_in_parameters = commands
        .iter()
        .any(|c| c.contains(FIX_ARGUMENT_PLACE_HOLDER) || c.contains(ITERATOR_ARGUMENT_PLACE_HOLDER));
    if has_std_in_parameters {
        execute_stdin(connection, commands, exec_command.output);
    } else {
        execute(connection, commands, exec_command.output);
    }
}

fn execute(con: &mut Connection, command: Vec<&str>, output_format: String) {
    if command.len() == 0 {
        return;
    }
    let name = command[0];
    let mut cmd = &mut redis::cmd(name);
    for &c in command.iter().skip(1) {
        cmd = cmd.arg(c)
    }
    match cmd.query::<String>(con) {
        Ok(r) => output(&String::new(), format!("{}", r), &output_format),
        Err(e) => {
            writeln_to_stderr(e.to_string());
            exit(1)
        },
    }
}

fn execute_stdin(con: &mut Connection, command: Vec<&str>, output_format: String) {
    if command.len() == 0 {
        return;
    }
    let name = command[0];
    let std_in = stdin();
    let mut continue_reading = true;
    let mut stdin_parameters: String = String::new();
    while continue_reading {
        let mut cmd = &mut redis::cmd(name);
        let mut parameter_fetched = false;
        stdin_parameters.clear();
        for &c in command.iter().skip(1) {
            if c.contains(FIX_ARGUMENT_PLACE_HOLDER) {
                if !parameter_fetched {
                    stdin_parameters.clear();
                    let i = read_stdin(&std_in, &mut stdin_parameters);
                    stdin_parameters = stdin_parameters.replace("\n", "");
                    if i == 0 {
                        continue_reading = false;
                    }
                    parameter_fetched = true;
                }
                cmd = cmd.arg(stdin_parameters.as_str());
            } else if c.contains(ITERATOR_ARGUMENT_PLACE_HOLDER) {
                stdin_parameters.clear();
                let i = read_stdin(&std_in, &mut stdin_parameters);
                stdin_parameters = stdin_parameters.replace("\n", "");
                if i == 0 {
                    continue_reading = false;
                }
                cmd = cmd.arg(stdin_parameters.as_str());
            } else {
                cmd = cmd.arg(c);
            }
        }
        if continue_reading {
            match cmd.query::<String>(con) {
                Ok(r) => output(&stdin_parameters, format!("{}", r), &output_format),
                Err(e) => {
                    writeln_to_stderr(e.to_string());
                    exit(1)
                }
            }
        }
    }
}

fn read_stdin(std_in: &Stdin, stdin_parameters: &mut String) -> usize {
    match std_in.read_line(stdin_parameters) {
        Ok(i) => i,
        Err(e) => {
            writeln_to_stderr(format!("Failed to read next stdin line : {}", e.to_string()).to_string());
            exit(1);
        }
    }
}

fn output(input_value: &String, output_value: String, output_format: &String) {
    let mut output_str = output_format.clone();
    output_str = output_str.replace("{stdout}", output_value.as_str());
    output_str = output_str.replace("{stdin}", input_value.as_str());
    writeln_to_stdout(output_str);
}