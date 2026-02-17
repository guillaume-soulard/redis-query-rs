use crate::io::writeln_to_stderr;
use crate::pipeline_executor;
use redis::Connection;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::env::home_dir;
use std::fs::{exists, File};
use std::process::exit;

pub fn shell(connection: &mut Connection, host: String, port: u16, db: u8) {
    let mut current_db = db;
    let mut pipeline_executor =
        pipeline_executor::PipelineExecutor::new(1, connection, String::from("{stdout}"), false, true);
    let mut rl = DefaultEditor::new().unwrap();

    let history_file_path = create_if_needed_and_get_history_file();
    match rl.load_history(history_file_path.as_str()) {
        Ok(_) => {}
        Err(e) => {
            writeln_to_stderr(format!("error loading history : {}", e));
            exit(1);
        }
    };
    loop {
        let readline = rl.readline(get_shell_prompt(&host, port, current_db).as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let cmd = line
                        .split(' ')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                if !cmd.is_empty() {
                    let command = cmd[0].to_lowercase();
                    match command.trim() {
                        "" => {}
                        "exit" => {
                            exit(0);
                        }
                        "select" => {
                            current_db = match cmd[1].parse::<u8>() {
                                Ok(db) => {
                                    pipeline_executor.execute(String::new(), cmd);
                                    pipeline_executor.flush();
                                    db
                                }
                                Err(e) => {
                                    writeln_to_stderr(
                                        format!("error parsing db : {}", e.to_string()).to_string(),
                                    );
                                    0
                                }
                            };
                        }
                        _ => {
                            pipeline_executor.execute(String::new(), cmd);
                        }
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                writeln_to_stderr(err.to_string());
                break
            }
        }
    }
    match rl.save_history(history_file_path.as_str()) {
        Ok(_) => {},
        Err(e) => {
            writeln_to_stderr(format!("error saving history: {}", e).to_string());
            exit(1);
        }
    };
}

fn create_if_needed_and_get_history_file() -> String {
    let mut option_directory = match home_dir() {
        Some(dir) => dir,
        None => {
            writeln_to_stderr("No home directory".to_string());
            exit(1);
        }
    };
    option_directory.push(".redis-query");
    option_directory.push("history");
    if !match exists(option_directory.as_path()) {
        Ok(file_exists) => file_exists,
        Err(e) => {
            writeln_to_stderr(format!("error checking history file existence: {}", e).to_string());
            exit(1);
        },
    } {
        match File::create_new(option_directory.as_path()) {
            Ok(_) => {},
            Err(e) => {
                writeln_to_stderr(format!("error creating history file: {}", e).to_string());
                exit(1);
            }
        };
    }
    let history_file_path = match option_directory.as_os_str().to_str() {
        Some(dir) => dir,
        None => {
            writeln_to_stderr("no path for history file".to_string());
            exit(1);
        }
    };
    history_file_path.to_string()
}

fn get_shell_prompt(host: &String, port: u16, db: u8) -> String {
    let mut db_str = String::new();
    if db != 0 {
        db_str.push_str(format!("[{}]", db).as_str());
    }
    format!("{}:{}{}> ", host, port, db_str)
}
