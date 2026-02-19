use crate::env::{create_if_needed_and_get_config_dir, load_env, Environment};
use crate::io::{writeln_to_stderr, writeln_to_stdout};
use crate::parameters::{
    DescribeEnvSubCommand,
    RemoveEnvSubCommand, SetEnvSubCommand
    ,
};
use std::fs;
use std::fs::{remove_file, File, OpenOptions};
use std::io::Write;
use std::process::exit;

pub fn set_env(set_env_command: SetEnvSubCommand) {
    let mut option_file = create_if_needed_and_get_config_dir();
    let env_name = set_env_command.name.clone();
    option_file.push(format!("{}.json", env_name));
    match remove_file(&option_file) {
        Ok(_) => {}
        Err(e) => {
            writeln_to_stderr(format!("Failed to remove env file : {}", e.to_string()).to_string());
            exit(1);
        }
    }
    match File::create(&option_file) {
        Ok(_) => {}
        Err(e) => {
            writeln_to_stderr(format!("Failed to create env file : {}", e.to_string()).to_string());
            exit(1);
        }
    };
    let env = Environment {
        name: env_name.clone(),
        host: set_env_command.host,
        db: set_env_command.db,
        port: set_env_command.port,
        sentinel_master: set_env_command.sentinel_master,
        user: set_env_command.user,
        password: set_env_command.password,
        sentinel_addrs: set_env_command.sentinel_addrs,
    };
    let json = match serde_json::to_string_pretty(&env) {
        Ok(json) => json,
        Err(e) => {
            writeln_to_stderr(
                format!(
                    "Failed to serialize env file {} : {}",
                    env_name,
                    e.to_string()
                )
                .to_string(),
            );
            exit(1);
        }
    };
    let mut file = match OpenOptions::new().write(true).open(option_file) {
        Ok(file) => file,
        Err(e) => {
            writeln_to_stderr(
                format!("Failed to open env file {} : {}", env_name, e.to_string()).to_string(),
            );
            exit(1);
        }
    };
    match file.write_all(json.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            writeln_to_stderr(format!("Failed to write env file : {}", e.to_string()).to_string());
            exit(1);
        }
    };
}

pub fn remove_env(remove_env_sub_command: RemoveEnvSubCommand) {
    let mut option_file = create_if_needed_and_get_config_dir();
    option_file.push(format!("{}.json", remove_env_sub_command.name));
    if option_file.exists() {
        match remove_file(option_file) {
            Ok(_) => {}
            Err(e) => {
                writeln_to_stderr(
                    format!("Failed to remove env file : {}", e.to_string()).to_string(),
                );
                exit(1);
            }
        };
    }
}

pub fn list_env() {
    let option_dir = create_if_needed_and_get_config_dir();
    let paths = match fs::read_dir(option_dir) {
        Ok(paths) => paths,
        Err(e) => {
            writeln_to_stderr(format!("Failed to read dir : {}", e.to_string()).to_string());
            exit(1);
        }
    };
    for path in paths {
        writeln_to_stdout(format!(
            "{}",
            path.unwrap().path().file_stem().unwrap().display()
        ));
    }
}

pub fn describe_env(env_describe_command: DescribeEnvSubCommand) {
    let mut option_file = create_if_needed_and_get_config_dir();
    option_file.push(format!("{}.json", env_describe_command.name));
    if !option_file.exists() {
        writeln_to_stdout(format!(
            "environment '{}' not found",
            env_describe_command.name
        ));
        return;
    }
    let env = load_env(env_describe_command.name);
    env.describe();
}
