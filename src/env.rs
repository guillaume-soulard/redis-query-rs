use crate::io::writeln_to_stdout;
use crate::parameters::{Connectable, DescribeEnvSubCommand, RemoveEnvSubCommand, SetEnvSubCommand};
use std::env::home_dir;
use std::fs;
use std::fs::{create_dir, remove_file, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

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
    let env = load_env(cmd.get_env());
    cmd.set_host(env.host);
    cmd.set_db(env.db);
    cmd.set_port(env.port);
    if env.user != "" { cmd.set_user(env.user); }
    cmd.set_password(env.password);
    cmd.set_sentinel_master(env.sentinel_master);
    cmd.set_sentinel_addrs(env.sentinel_addrs);
}

fn create_if_needed_and_get_config_dir() -> PathBuf {
    let mut option_directory = home_dir().expect("Failed to get home dir");
    option_directory.push(".redis-query");
    if !option_directory.exists() {
        create_dir(&option_directory).expect("Failed to create option directory");
    }
    option_directory
}

fn load_env(env_name: String) -> Environment {
    let mut option_file = create_if_needed_and_get_config_dir();
    option_file.push(format!("{}.json", env_name));
    let mut file = File::open(option_file).expect("Failed to open env file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read env file");
    let mut env: Environment = serde_json::from_str(&content).expect("Failed to parse env file");
    env.name = env_name;
    env
}

pub fn set_env(set_env_command: SetEnvSubCommand) {
    let mut option_file = create_if_needed_and_get_config_dir();
    option_file.push(format!("{}.json", set_env_command.name));
    if !option_file.exists() {
        File::create(&option_file).expect("Failed to create env file");
    }
    let env = Environment {
        name: set_env_command.name,
        host: set_env_command.host,
        db: set_env_command.db,
        port: set_env_command.port,
        sentinel_master: set_env_command.sentinel_master,
        user: set_env_command.user,
        password: set_env_command.password,
        sentinel_addrs: set_env_command.sentinel_addrs,
    };
    let json = serde_json::to_string_pretty(&env)
        .expect("Failed to serialize env file");
    let mut file = OpenOptions::new()
        .write(true)
        .open(option_file)
        .expect("Failed to open env file");
    file.write_all(json.as_bytes()).expect("Failed to write env file");
}

pub fn remove_env(remove_env_sub_command: RemoveEnvSubCommand) {
    let mut option_file = create_if_needed_and_get_config_dir();
    option_file.push(format!("{}.json", remove_env_sub_command.name));
    if option_file.exists() {
        remove_file(option_file).expect("Failed to remove env file");
    }
}

pub fn list_env() {
    let option_dir = create_if_needed_and_get_config_dir();
    let paths = fs::read_dir(option_dir).expect("Failed to read dir");
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