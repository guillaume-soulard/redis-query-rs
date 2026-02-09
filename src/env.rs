use crate::io::{writeln_to_stderr, writeln_to_stdout};
use crate::parameters::{Connectable, DescribeEnvSubCommand, RemoveEnvSubCommand, SetEnvSubCommand};
use std::env::home_dir;
use std::fs;
use std::fs::{create_dir, remove_file, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Environment {
    #[serde(skip)]
    name: String,
    host: String,
    db: u8,
    port: u16,
    sentinel_master: String,
    user: String,
    password: String,
    sentinel_addrs: String,
}

impl Environment {
    pub fn describe(&self) {
        writeln_to_stdout(format!("Environment: {}", self.name));
        writeln_to_stdout(format!("Host: {}", self.host));
        writeln_to_stdout(format!("port: {}", self.port));
        writeln_to_stdout(format!("db: {}", self.db));
        writeln_to_stdout(format!("user: {}", self.user));
        writeln_to_stdout(format!("password: {}", self.password));
        writeln_to_stdout(format!("sentinel_addrs: {}", self.sentinel_addrs));
        writeln_to_stdout(format!("sentinel_master: {}", self.sentinel_master));
    }
}

pub fn load_env_parameters(cmd: &mut dyn Connectable) {
    let env_name = cmd.get_env();
    if env_name == "" {
        return;
    }
    let env = load_env(env_name);
    cmd.set_host(env.host);
    cmd.set_db(env.db);
    cmd.set_port(env.port);
    if env.user != "" { cmd.set_user(env.user); }
    cmd.set_password(env.password);
    cmd.set_sentinel_master(env.sentinel_master);
    cmd.set_sentinel_addrs(env.sentinel_addrs);
}

fn create_if_needed_and_get_config_dir() -> PathBuf {
    let mut option_directory = match home_dir() {
        Some(dir) => dir,
        None => {
            writeln_to_stderr("No home directory".to_string());
            exit(1);
        }
    };
    option_directory.push(".redis-query");
    if !option_directory.exists() {
        match create_dir(&option_directory){
            Ok(_) => {},
            Err(e) => {
                writeln_to_stderr(format!("Failed to create option directory : {}", e.to_string()).to_string());
                exit(1);
            }
        };
    }
    option_directory
}

fn load_env(env_name: String) -> Environment {
    let mut option_file = create_if_needed_and_get_config_dir();
    option_file.push(format!("{}.json", env_name));
    let mut file = match File::open(option_file){
        Ok(file) => file,
        Err(e) => {
            writeln_to_stderr(format!("Failed to open env file {} : {}", env_name, e.to_string()).to_string());
            exit(1);
        }
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {},
        Err(e) => {
            writeln_to_stderr(format!("Failed to read env file {} : {}", env_name, e.to_string()).to_string());
            exit(1);
        }
    }
    let mut env: Environment = match serde_json::from_str(&content){
        Ok(env) => env,
        Err(e) => {
            writeln_to_stderr(format!("Failed to parse env file {} : {}", env_name, e.to_string()).to_string());
            exit(1);
        }
    };
    env.name = env_name;
    env
}

pub fn set_env(set_env_command: SetEnvSubCommand) {
    let mut option_file = create_if_needed_and_get_config_dir();
    let env_name = set_env_command.name.clone();
    option_file.push(format!("{}.json", env_name));
    if !option_file.exists() {
        match File::create(&option_file) {
            Ok(_) => {},
            Err(e) => {
                writeln_to_stderr(format!("Failed to create env file : {}", e.to_string()).to_string());
                exit(1);
            }
        };
    }
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
            writeln_to_stderr(format!("Failed to serialize env file {} : {}", env_name, e.to_string()).to_string());
            exit(1);
        },
    };
    let mut file = match OpenOptions::new()
        .write(true)
        .open(option_file) {
            Ok(file) => file,
            Err(e) => {
                writeln_to_stderr(format!("Failed to open env file {} : {}", env_name, e.to_string()).to_string());
                exit(1);
            }
        };
    match file.write_all(json.as_bytes()) {
        Ok(_) => {},
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
            Ok(_) => {},
            Err(e) => {
                writeln_to_stderr(format!("Failed to remove env file : {}", e.to_string()).to_string());
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
        },
    };
    for path in paths {
        writeln_to_stdout(format!("{}", path.unwrap().path().file_stem().unwrap().display()));
    }
}

pub fn describe_env(env_describe_command: DescribeEnvSubCommand) {
    let mut option_file = create_if_needed_and_get_config_dir();
    option_file.push(format!("{}.json", env_describe_command.name));
    if !option_file.exists() {
        writeln_to_stdout(format!("environment '{}' not found", env_describe_command.name));
        return;
    }
    let env = load_env(env_describe_command.name);
    env.describe();
}