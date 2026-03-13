use crate::io::{writeln_redis_value_to_stdout, writeln_redis_value_to_stdout_for_cli, writeln_to_stdout};
use redis::{Connection, ErrorKind, ServerErrorKind, Value};
use std::collections::LinkedList;
use std::process::exit;

pub struct PipelineExecutor<'a> {
    max_pipeline_size: usize,
    commands: LinkedList<(String, Vec<String>)>,
    connection: &'a mut Connection,
    output_format: String,
    exit_on_error: bool,
    cli_output: bool,
}

impl<'a> PipelineExecutor<'a> {
    pub fn new(max_pipeline_size: usize, connection: &'a mut Connection, output_format: String, exit_on_error: bool, cli_output: bool) -> Self {
        PipelineExecutor {
            max_pipeline_size,
            commands: LinkedList::new(),
            connection,
            output_format,
            exit_on_error,
            cli_output,
        }
    }
    pub fn execute(&mut self, stdin: String, command_to_add: Vec<String>) {
        if self.max_pipeline_size == 1 {
            if command_to_add.len() == 0 {
                return
            }
            let mut cmd = redis::cmd(command_to_add[0].as_str());
            for c in command_to_add.iter().skip(1) {
                cmd.arg(c.as_str());
            }
            match cmd.query::<Value>(self.connection) {
                Ok(value) => {
                    if self.cli_output {
                        writeln_redis_value_to_stdout_for_cli(value);
                    } else {
                        writeln_redis_value_to_stdout(&stdin, value, &self.output_format);
                    }
                },
                Err(e) => {
                    match e.kind() {
                        ErrorKind::Server(ServerErrorKind::Moved) => {
                            todo
                            writeln_to_stdout("moved".to_string());
                        }
                        ErrorKind::Server(ServerErrorKind::Ask) => {
                            todo
                            writeln_to_stdout("ask".to_string());
                        }
                        _ => {
                            writeln_to_stdout(format!("{}", e.to_string()));
                        }
                    }
                }
            };
        } else {
            self.commands.push_back((stdin, command_to_add));
            if self.commands.len() < self.max_pipeline_size {
                return;
            }
            self.flush();
        }
    }
    pub fn flush(&mut self) {
        if self.commands.len() == 0 {
            return;
        }
        let mut pipeline = redis::pipe();
        let mut std_in = Vec::new();
        for command in self.commands.iter() {
            if command.1.len() == 0 {
                continue;
            }
            pipeline.cmd(command.1[0].as_str());
            for c in command.1.iter().skip(1) {
                pipeline.arg(c.as_str());
            }
            std_in.push(command.0.clone());
        }
        self.commands.clear();
        match pipeline.query::<Value>(self.connection) {
            Ok(Value::Array(a)) => {
                let mut i = 0;
                for v in a {
                    if self.cli_output {
                        writeln_redis_value_to_stdout_for_cli(v);
                    } else {
                        writeln_redis_value_to_stdout(&std_in[i], v, &self.output_format);
                    }
                    i += 1;
                }
            },
            Ok(_) => {
                writeln_to_stdout("pipeline execution failed : wrong return type. Expected array response for pipeline execution".to_string());
                if self.exit_on_error {
                    exit(1);
                }
            }
            Err(e) => {
                writeln_to_stdout(format!("pipeline execution failed : {}", e.to_string()).to_string());
                if self.exit_on_error {
                    exit(1);
                }
            }
        }
    }
}