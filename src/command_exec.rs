use crate::io::writeln_to_stderr;
use crate::parameters::ExecSubCommand;
use crate::pipeline_executor;
use redis::Connection;
use std::io::{Stdin, stdin};
use std::process::exit;

const FIX_ARGUMENT_PLACE_HOLDER: &'static str = "{?}";
const ITERATOR_ARGUMENT_PLACE_HOLDER: &'static str = "{>}";

pub fn exec_command(connection: &mut Connection, exec_command: ExecSubCommand) {
    let commands = exec_command.command.split(' ').collect::<Vec<&str>>();
    let has_std_in_parameters = commands.iter().any(|c| {
        c.contains(FIX_ARGUMENT_PLACE_HOLDER) || c.contains(ITERATOR_ARGUMENT_PLACE_HOLDER)
    });
    if has_std_in_parameters {
        execute_stdin(connection, commands, &exec_command);
    } else {
        execute(connection, commands, &exec_command);
    }
}

fn execute(con: &mut Connection, command: Vec<&str>, exec_sub_command: &ExecSubCommand) {
    if command.len() == 0 {
        return;
    }
    let name = command[0].to_string();
    let mut cmd = Vec::new();
    cmd.push(name);
    for &c in command.iter().skip(1) {
        cmd.push(c.to_string())
    }
    let mut pipeline_executor = pipeline_executor::PipelineExecutor::new(
        exec_sub_command.pipeline,
        con,
        exec_sub_command.output.clone(),
    );
    pipeline_executor.execute(String::new(), cmd);
    pipeline_executor.flush();
}

fn execute_stdin(con: &mut Connection, command: Vec<&str>, exec_sub_command: &ExecSubCommand) {
    if command.len() == 0 {
        return;
    }
    let std_in = stdin();
    let mut continue_reading = true;
    let mut stdin_parameters: String = String::new();
    let mut pipeline_executor = pipeline_executor::PipelineExecutor::new(
        exec_sub_command.pipeline,
        con,
        exec_sub_command.output.clone(),
    );
    while continue_reading {
        let mut cmd = Vec::new();
        cmd.push(command[0].to_string());
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
                cmd.push(stdin_parameters.clone());
            } else if c.contains(ITERATOR_ARGUMENT_PLACE_HOLDER) {
                stdin_parameters.clear();
                let i = read_stdin(&std_in, &mut stdin_parameters);
                stdin_parameters = stdin_parameters.replace("\n", "");
                if i == 0 {
                    continue_reading = false;
                }
                cmd.push(stdin_parameters.clone());
            } else {
                cmd.push(c.to_string());
            }
        }
        if continue_reading {
            pipeline_executor.execute(stdin_parameters.clone(), cmd);
        }
    }
    pipeline_executor.flush();
}

fn read_stdin(std_in: &Stdin, stdin_parameters: &mut String) -> usize {
    match std_in.read_line(stdin_parameters) {
        Ok(i) => i,
        Err(e) => {
            writeln_to_stderr(
                format!("Failed to read next stdin line : {}", e.to_string()).to_string(),
            );
            exit(1);
        }
    }
}
