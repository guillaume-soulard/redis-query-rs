use crate::io::writeln_to_stderr;
use clap::{Args, Parser, Subcommand};
use std::process::exit;

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

#[derive(Args, Debug)]
pub struct ScanSubCommand {
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
    #[arg(
        short = 'i',
        long = "instance",
        required = false,
        default_value = ""
    )]
    pub connection_string: String,
}

#[derive(Args, Debug)]
pub struct ExecSubCommand {
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
    #[arg(
        short = 'i',
        long = "instance",
        required = false,
        default_value = ""
    )]
    pub connection_string: String,
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
        short = 'i',
        long = "instance",
        required = false,
        default_value = ""
    )]
    pub connection_string: String,
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

#[derive(Args, Debug)]
pub struct ConnectSubCommand {
    #[arg(
        conflicts_with_all = ["host","port","db","user","password","sentinel_addrs","sentinel_master"],
        short = 'e',
        long = "env",
        required = false,
        default_value = ""
    )]
    pub env: String,
    #[arg(
        short = 'i',
        long = "instance",
        required = false,
        default_value = ""
    )]
    pub connection_string: String,
}
