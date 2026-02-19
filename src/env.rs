use std::env::home_dir;
use std::fs::{create_dir, File};
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;
use crate::io::{writeln_to_stderr, writeln_to_stdout};
use crate::parameters::{Connectable, DEFAULT_SENTINEL_ADDRS, DEFAULT_SENTINEL_MASTER};

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Environment {
    #[serde(skip)]
    pub name: String,
    pub host: String,
    pub db: u8,
    pub port: u16,
    pub sentinel_master: String,
    pub user: String,
    pub password: String,
    pub sentinel_addrs: String,
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

pub fn load_env_parameters(env_name: String, cmd: &mut dyn Connectable) {
    if env_name == "" {
        return;
    }
    let env = load_env(env_name);
    cmd.set_host(env.host);
    cmd.set_db(env.db);
    cmd.set_port(env.port);
    cmd.set_user(env.user);
    cmd.set_password(env.password);
    if cmd.get_sentinel_master() == DEFAULT_SENTINEL_MASTER
        || env.sentinel_master != ""
        || env.sentinel_master != DEFAULT_SENTINEL_MASTER
    {
        cmd.set_sentinel_master(env.sentinel_master);
    }
    if cmd.get_sentinel_addrs() == DEFAULT_SENTINEL_ADDRS
        || env.sentinel_addrs != ""
        || env.sentinel_addrs != DEFAULT_SENTINEL_ADDRS
    {
        cmd.set_sentinel_addrs(env.sentinel_addrs);
    }
}

pub fn load_env(env_name: String) -> Environment {
    let mut option_file = create_if_needed_and_get_config_dir();
    option_file.push(format!("{}.json", env_name));
    let mut file = match File::open(option_file) {
        Ok(file) => file,
        Err(e) => {
            writeln_to_stderr(
                format!("Failed to open env file {} : {}", env_name, e.to_string()).to_string(),
            );
            exit(1);
        }
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(e) => {
            writeln_to_stderr(
                format!("Failed to read env file {} : {}", env_name, e.to_string()).to_string(),
            );
            exit(1);
        }
    }
    let mut env: Environment = match serde_json::from_str(&content) {
        Ok(env) => env,
        Err(e) => {
            writeln_to_stderr(
                format!("Failed to parse env file {} : {}", env_name, e.to_string()).to_string(),
            );
            exit(1);
        }
    };
    env.name = env_name;
    env
}

pub fn create_if_needed_and_get_config_dir() -> PathBuf {
    let mut option_directory = match home_dir() {
        Some(dir) => dir,
        None => {
            writeln_to_stderr("No home directory".to_string());
            exit(1);
        }
    };
    option_directory.push(".redis-query");
    if !option_directory.exists() {
        match create_dir(&option_directory) {
            Ok(_) => {}
            Err(e) => {
                writeln_to_stderr(
                    format!("Failed to create option directory : {}", e.to_string()).to_string(),
                );
                exit(1);
            }
        };
    }
    option_directory
}
