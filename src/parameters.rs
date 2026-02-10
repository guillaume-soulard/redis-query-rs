use crate::io::{writeln_to_stderr};
use clap::{Args, Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct RqParameters {
    #[command(subcommand)]
    pub command: RqSubCommand,
}

pub fn load_parameters() -> RqParameters {
    match RqParameters::try_parse() {
        Ok(params) => params,
        Err(err) => {
            writeln_to_stderr(err.to_string());
            exit(1);
        }
    }
}

#[derive(Subcommand)]
pub enum RqSubCommand {
    Scan(ScanSubCommand),
    Exec(ExecSubCommand),
    #[command(subcommand)]
    Env(EnvSubCommand),
    Migrate(MigrateSubCommand),
}

pub trait Connectable {
    fn get_host(&self) -> String;
    fn set_host(&mut self, host: String);
    fn get_port(&self) -> u16;
    fn set_port(&mut self, port: u16);
    fn get_db(&self) -> u8;
    fn set_db(&mut self, db: u8);
    fn get_user(&self) -> String;
    fn set_user(&mut self, user: String);
    fn get_password(&self) -> String;
    fn set_password(&mut self, password: String);
    fn get_sentinel_master(&self) -> String;
    fn set_sentinel_master(&mut self, sentinel_master: String);
    fn get_sentinel_addrs(&self) -> String;
    fn set_sentinel_addrs(&mut self, sentinel_addrs: String);
}

#[derive(Args)]
pub struct ScanSubCommand {
    #[arg(
        short = 'H',
        long = "host",
        required = false,
        default_value = "localhost"
    )]
    pub host: String,
    #[arg(short = 'p', long = "port", required = false, default_value = "6379")]
    pub port: u16,
    #[arg(short = 'd', long = "db", required = false, default_value = "0")]
    pub db: u8,
    #[arg(
        short = 'u',
        long = "user",
        required = false,
        default_value = "default"
    )]
    pub user: String,
    #[arg(short = 'w', long = "pass", required = false, default_value = "")]
    pub password: String,
    #[arg(long = "sentinel-master", required = false, default_value = "mymaster")]
    pub sentinel_master: String,
    #[arg(long = "sentinel-addrs", required = false, default_value = "localhost:26379")]
    pub sentinel_addrs: String,
    #[arg(long = "pattern", required = true)]
    pub pattern: String,
    #[arg(short = 'c', long = "count", required = false, default_value = "10")]
    pub count: usize,
    #[arg(
        short = 'l',
        long = "limit",
        required = false,
        default_value = "18446744073709551615"
    )]
    pub limit: usize,
    #[arg(short = 'e', long = "env", required = false, default_value = "")]
    pub env: String,
}

impl Connectable for ScanSubCommand {
    fn get_host(&self) -> String {
        self.host.clone()
    }
    fn set_host(&mut self, host: String) {
        self.host = host;
    }
    fn get_port(&self) -> u16 {
        self.port.clone()
    }
    fn set_port(&mut self, port: u16) {
        self.port = port;
    }
    fn get_db(&self) -> u8 {
        self.db.clone()
    }
    fn set_db(&mut self, db: u8) {
        self.db = db;
    }
    fn get_user(&self) -> String {
        self.user.clone()
    }
    fn set_user(&mut self, user: String) {
        self.user = user;
    }
    fn get_password(&self) -> String {
        self.password.clone()
    }
    fn set_password(&mut self, password: String) {
        self.password = password;
    }
    fn get_sentinel_master(&self) -> String {
        self.sentinel_master.clone()
    }
    fn set_sentinel_master(&mut self, sentinel_master: String) {
        self.sentinel_master = sentinel_master;
    }
    fn get_sentinel_addrs(&self) -> String {
        self.sentinel_addrs.clone()
    }
    fn set_sentinel_addrs(&mut self, sentinel_addrs: String) {
        self.sentinel_addrs = sentinel_addrs;
    }
}

#[derive(Args)]
pub struct ExecSubCommand {
    #[arg(
        short = 'H',
        long = "host",
        required = false,
        default_value = "localhost"
    )]
    pub host: String,
    #[arg(short = 'p', long = "port", required = false, default_value = "6379")]
    pub port: u16,
    #[arg(short = 'd', long = "db", required = false, default_value = "0")]
    pub db: u8,
    #[arg(
        short = 'u',
        long = "user",
        required = false,
        default_value = "default"
    )]
    pub user: String,
    #[arg(short = 'w', long = "pass", required = false, default_value = "")]
    pub password: String,
    #[arg(long = "sentinel-master", required = false, default_value = "mymaster")]
    pub sentinel_master: String,
    #[arg(long = "sentinel-addrs", required = false, default_value = "localhost:26379")]
    pub sentinel_addrs: String,
    #[arg(short = 'c', long = "command", required = true)]
    pub command: String,
    #[arg(short = 'e', long = "env", required = false, default_value = "")]
    pub env: String,
    #[arg(short = 'o', long = "output", required = false, default_value = "{stdout}")]
    pub output: String,
}

impl Connectable for ExecSubCommand {
    fn get_host(&self) -> String {
        self.host.clone()
    }

    fn set_host(&mut self, host: String) {
        self.host = host;
    }

    fn get_port(&self) -> u16 {
        self.port.clone()
    }

    fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    fn get_db(&self) -> u8 {
        self.db.clone()
    }

    fn set_db(&mut self, db: u8) {
        self.db = db;
    }

    fn get_user(&self) -> String {
        self.user.clone()
    }

    fn set_user(&mut self, user: String) {
        self.user = user;
    }

    fn get_password(&self) -> String {
        self.password.clone()
    }

    fn set_password(&mut self, password: String) {
        self.password = password;
    }

    fn get_sentinel_master(&self) -> String {
        self.sentinel_master.clone()
    }

    fn set_sentinel_master(&mut self, sentinel_master: String) {
        self.sentinel_master = sentinel_master;
    }

    fn get_sentinel_addrs(&self) -> String {
        self.sentinel_addrs.clone()
    }

    fn set_sentinel_addrs(&mut self, sentinel_addrs: String) {
        self.sentinel_addrs = sentinel_addrs;
    }
}

#[derive(Subcommand)]
pub enum EnvSubCommand {
    Set(SetEnvSubCommand),
    List(ListEnvSubCommand),
    Remove(RemoveEnvSubCommand),
    Describe(DescribeEnvSubCommand),
}

#[derive(Args)]
pub struct SetEnvSubCommand {
    #[arg(short = 'n', long = "name", required = true)]
    pub name: String,
    #[arg(
        short = 'H',
        long = "host",
        required = false,
        default_value = "localhost"
    )]
    pub host: String,
    #[arg(short = 'p', long = "port", required = false, default_value = "6379")]
    pub port: u16,
    #[arg(short = 'd', long = "db", required = false, default_value = "0")]
    pub db: u8,
    #[arg(
        short = 'u',
        long = "user",
        required = false,
        default_value = "default"
    )]
    pub user: String,
    #[arg(short = 'w', long = "pass", required = false, default_value = "")]
    pub password: String,
    #[arg(long = "sentinel-master", required = false, default_value = "mymaster")]
    pub sentinel_master: String,
    #[arg(long = "sentinel-addrs", required = false, default_value = "localhost:26379")]
    pub sentinel_addrs: String,
}

#[derive(Args)]
pub struct ListEnvSubCommand {}

#[derive(Args)]
pub struct RemoveEnvSubCommand {
    #[arg(short = 'n', long = "name", required = true)]
    pub name: String,
}

#[derive(Args)]
pub struct DescribeEnvSubCommand {
    #[arg(short = 'n', long = "name", required = true)]
    pub name: String,
}

#[derive(Args)]
pub struct MigrateSubCommand {
    #[arg(
        short = 'H',
        long = "host",
        required = false,
        default_value = "localhost"
    )]
    pub host: String,
    #[arg(short = 'p', long = "port", required = false, default_value = "6379")]
    pub port: u16,
    #[arg(short = 'd', long = "db", required = false, default_value = "0")]
    pub db: u8,
    #[arg(
        short = 'u',
        long = "user",
        required = false,
        default_value = "default"
    )]
    pub user: String,
    #[arg(short = 'w', long = "pass", required = false, default_value = "")]
    pub password: String,
    #[arg(long = "sentinel-master", required = false, default_value = "mymaster")]
    pub sentinel_master: String,
    #[arg(long = "sentinel-addrs", required = false, default_value = "localhost:26379")]
    pub sentinel_addrs: String,
    #[arg(short = 's', long = "source-env", required = true)]
    pub source_env: String,
    #[arg(short = 't', long = "target-env", required = true)]
    pub target_env: String,
    #[arg(short = 'c', long = "count", required = false, default_value = "10")]
    pub count: usize,
    #[arg(short = 'l', long = "limit", required = false, default_value = "18446744073709551615")]
    pub limit: usize,
    #[arg(short = 'r', long = "replace", required = false, default_value = "true")]
    pub replace: bool,
    #[arg(long = "ttl", required = false, default_value = "-3")]
    pub ttl: i64,
    #[arg(long = "source-pattern", required = true)]
    pub source_pattern: String,
}

impl Connectable for MigrateSubCommand {
    fn get_host(&self) -> String {
        self.host.clone()
    }

    fn set_host(&mut self, host: String) {
        self.host = host;
    }

    fn get_port(&self) -> u16 {
        self.port.clone()
    }

    fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    fn get_db(&self) -> u8 {
        self.db.clone()
    }

    fn set_db(&mut self, db: u8) {
        self.db = db;
    }

    fn get_user(&self) -> String {
        self.user.clone()
    }

    fn set_user(&mut self, user: String) {
        self.user = user;
    }

    fn get_password(&self) -> String {
        self.password.clone()
    }

    fn set_password(&mut self, password: String) {
        self.password = password;
    }

    fn get_sentinel_master(&self) -> String {
        self.sentinel_master.clone()
    }

    fn set_sentinel_master(&mut self, sentinel_master: String) {
        self.sentinel_master = sentinel_master;
    }

    fn get_sentinel_addrs(&self) -> String {
        self.sentinel_addrs.clone()
    }

    fn set_sentinel_addrs(&mut self, sentinel_addrs: String) {
        self.sentinel_addrs = sentinel_addrs;
    }
}
