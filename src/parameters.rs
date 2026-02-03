use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct RqParameters {
    #[command(subcommand)]
    pub command: RqSubCommand,
}


pub fn parse_parameters() -> RqParameters {
    RqParameters::try_parse()
        .unwrap_or_else(|error| { panic!("error parsing arguments {}", error.to_string()) })
}

#[derive(Subcommand)]
pub enum RqSubCommand {
    Exec(ExecSubCommand),
}

#[derive(Args)]
pub struct ExecSubCommand {
    #[arg(short = 'H', long = "host", required = false, default_value = "localhost")]
    pub host: String,
    #[arg(short = 'p', long = "port", required = false, default_value = "6379")]
    pub port: u16,
    #[arg(short = 'd', long = "db", required = false, default_value = "0")]
    pub db: u8,
    #[arg(short = 'u', long = "user", required = false, default_value = "default")]
    pub user: String,
    #[arg(short = 'w', long = "pass", required = false, default_value = "")]
    pub password: String,
    #[arg(long = "sentinel-master", required = false, default_value = "mymaster")]
    pub sentinel_master: String,
    #[arg(long = "sentinel-addr", required = false, default_value = "localhost")]
    pub sentinel_addr: String,
    #[arg(short = 'c', long = "command", required = true)]
    pub command: String,
}
