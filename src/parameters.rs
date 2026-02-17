use crate::io::writeln_to_stderr;
use clap::{Args, Parser, Subcommand};
use std::process::exit;

pub const DEFAULT_HOST:&str = "localhost";
pub const DEFAULT_PORT:&str = "6379";
pub const DEFAULT_DB:&str = "0";
pub const DEFAULT_USER:&str = "default";
pub const DEFAULT_PASSWORD:&str = "";
pub const DEFAULT_SENTINEL_ADDRS:&str = "";
pub const DEFAULT_SENTINEL_MASTER:&str = "";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
pub enum RqSubCommand {
    /// Iteratively scan a redis instance
    Scan(ScanSubCommand),
    /// Execute a command on a redis instance
    Exec(ExecSubCommand),
    #[command(subcommand)]
    /// Manage environment
    Env(EnvSubCommand),
    /// Copy keys based on a pattern from one redis instance to another
    Copy(MigrateSubCommand),
    /// Connect to a redis instance
    Connect(ConnectSubCommand),
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

#[derive(Args, Debug)]
pub struct ScanSubCommand {
    #[arg(
        conflicts_with_all(["sentinel_master","sentinel_addrs","env"]),
        short = 'H',
        long = "host",
        required = false,
        default_value = DEFAULT_HOST
    )]
    pub host: String,
    #[arg(
        conflicts_with_all(["sentinel_master","sentinel_addrs","env"]), short = 'p', long = "port", required = false, default_value = DEFAULT_PORT)]
    pub port: u16,
    #[arg(conflicts_with_all = ["env"],short = 'd', long = "db", required = false, default_value = DEFAULT_DB)]
    pub db: u8,
    #[arg(
        conflicts_with_all = ["env"],
        short = 'u',
        long = "user",
        required = false,
        default_value = DEFAULT_USER
    )]
    pub user: String,
    #[arg(conflicts_with_all = ["env"],short = 'w', long = "password", required = false, default_value = DEFAULT_PASSWORD)]
    pub password: String,
    #[arg(
        conflicts_with_all = ["host","env"],
        long = "sentinel-master",
        required = false,
        default_value = DEFAULT_SENTINEL_MASTER
    )]
    pub sentinel_master: String,
    #[arg(
        conflicts_with_all = ["host","env"],
        long = "sentinel-addrs",
        required = false,
        default_value = DEFAULT_SENTINEL_ADDRS
    )]
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
    #[arg(
        conflicts_with_all = ["host","port","db","user","password","sentinel_addrs","sentinel_master"],
        short = 'e',
        long = "env",
        required = false,
        default_value = ""
    )]
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

#[derive(Args, Debug)]
pub struct ExecSubCommand {
    #[arg(
        conflicts_with_all(["sentinel_master","sentinel_addrs","env"]),
        short = 'H',
        long = "host",
        required = false,
        default_value = DEFAULT_HOST
    )]
    pub host: String,
    #[arg(conflicts_with_all(["sentinel_master","sentinel_addrs","env"]), short = 'p', long = "port", required = false, default_value = DEFAULT_PORT)]
    pub port: u16,
    #[arg(conflicts_with_all = ["env"],short = 'd', long = "db", required = false, default_value = DEFAULT_DB)]
    pub db: u8,
    #[arg(
        conflicts_with_all = ["env"],
        short = 'u',
        long = "user",
        required = false,
        default_value = DEFAULT_USER
    )]
    pub user: String,
    #[arg(conflicts_with_all = ["env"],short = 'w', long = "password", required = false, default_value = DEFAULT_PASSWORD)]
    pub password: String,
    #[arg(conflicts_with_all = ["host","env"],long = "sentinel-master", required = false, default_value = DEFAULT_SENTINEL_MASTER)]
    pub sentinel_master: String,
    #[arg(
        conflicts_with_all = ["host","env"],
        long = "sentinel-addrs",
        required = false,
        default_value = DEFAULT_SENTINEL_ADDRS
    )]
    pub sentinel_addrs: String,
    /// The command to execute. Can use spacial placeholders as parameters : {?} for current stdin value and {>} for next stdin value.
    #[arg(short = 'c', long = "command", required = true)]
    pub command: String,
    #[arg(
        conflicts_with_all = ["host","port","db","user","password","sentinel_addrs","sentinel_master"],
        short = 'e',
        long = "env",
        required = false,
        default_value = ""
    )]
    pub env: String,
    /// The format to use to output the result. Usable placeholders {stdin} or {stdout}
    #[arg(
        short = 'o',
        long = "output",
        required = false,
        default_value = "{stdout}"
    )]
    pub output: String,
    /// The number of commands to execute in one redis pipeline.
    #[arg(short = 'P', long = "pipeline", required = false, default_value = "1")]
    pub pipeline: usize,
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

#[derive(Subcommand, Debug)]
pub enum EnvSubCommand {
    Set(SetEnvSubCommand),
    List(ListEnvSubCommand),
    Remove(RemoveEnvSubCommand),
    Describe(DescribeEnvSubCommand),
}

#[derive(Args, Debug)]
pub struct SetEnvSubCommand {
    #[arg(short = 'n', long = "name", required = true)]
    pub name: String,
    #[arg(
        conflicts_with_all(["sentinel_master","sentinel_addrs"]),
        short = 'H',
        long = "host",
        required = false,
        default_value = ""
    )]
    pub host: String,
    #[arg(conflicts_with_all(["sentinel_master","sentinel_addrs"]), short = 'p', long = "port", required = false, default_value = DEFAULT_PORT)]
    pub port: u16,
    #[arg(short = 'd', long = "db", required = false, default_value = DEFAULT_DB)]
    pub db: u8,
    #[arg(
        short = 'u',
        long = "user",
        required = false,
        default_value = ""
    )]
    pub user: String,
    #[arg(short = 'w', long = "password", required = false, default_value = "")]
    pub password: String,
    #[arg(conflicts_with = "host",long = "sentinel-master", required = false, default_value = "")]
    pub sentinel_master: String,
    #[arg(
        conflicts_with = "host",
        long = "sentinel-addrs",
        required = false,
        default_value = ""
    )]
    pub sentinel_addrs: String,
}

#[derive(Args, Debug)]
pub struct ListEnvSubCommand {}

#[derive(Args, Debug)]
pub struct RemoveEnvSubCommand {
    #[arg(short = 'n', long = "name", required = true)]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct DescribeEnvSubCommand {
    #[arg(short = 'n', long = "name", required = true)]
    pub name: String,
}

#[derive(Args, Debug)]
pub struct MigrateSubCommand {
    #[arg(skip)]
    pub host: String,
    #[arg(skip)]
    pub port: u16,
    #[arg(skip)]
    pub db: u8,
    #[arg(skip)]
    pub user: String,
    #[arg(skip)]
    pub password: String,
    #[arg(skip)]
    pub sentinel_master: String,
    #[arg(skip)]
    pub sentinel_addrs: String,
    #[arg(short = 's', long = "source-env", required = true)]
    pub source_env: String,
    #[arg(short = 't', long = "target-env", required = true)]
    pub target_env: String,
    #[arg(short = 'c', long = "count", required = false, default_value = "10")]
    pub count: usize,
    #[arg(
        short = 'l',
        long = "limit",
        required = false,
        default_value = "18446744073709551615"
    )]
    pub limit: usize,
    #[arg(
        short = 'r',
        long = "replace",
        required = false,
        default_value = "true"
    )]
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

#[derive(Args, Debug)]
pub struct ConnectSubCommand {
    #[arg(
        conflicts_with_all(["sentinel_master","sentinel_addrs","env"]),
        short = 'H',
        long = "host",
        required = false,
        default_value = DEFAULT_HOST
    )]
    pub host: String,
    #[arg(conflicts_with_all(["sentinel_master","sentinel_addrs","env"]), short = 'p', long = "port", required = false, default_value = DEFAULT_PORT)]
    pub port: u16,
    #[arg(conflicts_with_all = ["env"],short = 'd', long = "db", required = false, default_value = DEFAULT_DB)]
    pub db: u8,
    #[arg(
        conflicts_with_all = ["env"],
        short = 'u',
        long = "user",
        required = false,
        default_value = DEFAULT_USER
    )]
    pub user: String,
    #[arg(conflicts_with_all = ["env"],short = 'w', long = "password", required = false, default_value = DEFAULT_PASSWORD)]
    pub password: String,
    #[arg(conflicts_with_all = ["host","env"],long = "sentinel-master", required = false, default_value = DEFAULT_SENTINEL_MASTER)]
    pub sentinel_master: String,
    #[arg(
        conflicts_with_all = ["host","env"],
        long = "sentinel-addrs",
        required = false,
        default_value = DEFAULT_SENTINEL_ADDRS
    )]
    pub sentinel_addrs: String,
    #[arg(
        conflicts_with_all = ["host","port","db","user","password","sentinel_addrs","sentinel_master"],
        short = 'e',
        long = "env",
        required = false,
        default_value = ""
    )]
    pub env: String,
}

impl Connectable for ConnectSubCommand {
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
