use crate::connection::{get_redis_connection_infos, RedisConnectionInfos};
use crate::io::{writeln_to_stderr, writeln_to_stdout};
use std::env::home_dir;
use std::fs::{create_dir, File};
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Environment {
    #[serde(skip)]
    pub name: String,
    pub connection_string: String,
}

impl Environment {
    pub fn describe(&self) {
        writeln_to_stdout(format!("Environment: {}", self.name));
        writeln_to_stdout(format!("Connection string: {}", self.connection_string));
    }
}

pub fn load_env_parameters_or_from_connection_string(env_name: String, connection_string: &String) -> RedisConnectionInfos {
    if env_name == "" {
        if connection_string == "" {
            writeln_to_stderr("No connection string specified".to_string());
            exit(1);
        }
        return get_redis_connection_infos(&connection_string);
    }
    let env = load_env(env_name);
    get_redis_connection_infos(&env.connection_string)
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
