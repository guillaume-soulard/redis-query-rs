use crate::io::{writeln_redis_value_to_stdout, writeln_to_stderr};
use redis::{Connection, Value};
use std::collections::LinkedList;
use std::process::exit;

pub struct PipelineExecutor<'a> {
    max_pipeline_size: usize,
    commands: LinkedList<(String, Vec<String>)>,
    connection: &'a mut Connection,
    output_format: String,
}

impl<'a> PipelineExecutor<'a> {
    pub fn new(max_pipeline_size: usize, connection: &'a mut Connection, output_format: String) -> Self {
        PipelineExecutor {
            max_pipeline_size,
            commands: LinkedList::new(),
            connection,
            output_format,
        }
    }
    pub fn execute(&mut self, stdin: String, command_to_add: Vec<String>) {
        self.commands.push_back((stdin, command_to_add));
        if self.commands.len() < self.max_pipeline_size {
                return;
        }
        self.flush();
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
                    writeln_redis_value_to_stdout(&std_in[i], v, &self.output_format);
                    i += 1;
                }
            },
            Ok(_) => {
                writeln_to_stderr("pipeline execution failed : wrong return type. Expected array response for pipeline execution".to_string());
                exit(1);
            }
            Err(e) => {
                writeln_to_stderr(format!("pipeline execution failed : {}", e.to_string()).to_string());
                exit(1);
            }
        }
    }
}