extern crate core;
mod parameters;
mod connection;
mod io;
mod command_copy;
mod command_scan;
mod command_exec;
mod command_env;
mod pipeline_executor;
mod command_connect;
mod env;

use crate::command_connect::shell;
use crate::command_copy::migrate;
use crate::command_env::{describe_env, list_env, remove_env, set_env};
use crate::command_exec::exec_command;
use crate::command_scan::scan_command;
use crate::connection::connect;
use crate::env::load_env_parameters_or_from_connection_string;
use crate::parameters::{load_parameters, EnvSubCommand, RqParameters, RqSubCommand};

fn main() {
    let parameters: RqParameters = load_parameters();
    match parameters.command {
        RqSubCommand::Exec(cmd) => {
            let connection_infos = load_env_parameters_or_from_connection_string(cmd.env.clone(), &cmd.connection_string);
            let mut con = connect(&connection_infos).connection;
            exec_command(&mut con, cmd);
        }
        RqSubCommand::Scan(scan) => {
            let connection_infos = load_env_parameters_or_from_connection_string(scan.env.clone(), &scan.connection_string);
            let mut con = connect(&connection_infos).connection;
            scan_command(&mut con, scan);
        },
        RqSubCommand::Env(env) => {
            match env {
                EnvSubCommand::Set(set_env_cmd) => set_env(set_env_cmd),
                EnvSubCommand::List(_) => list_env(),
                EnvSubCommand::Remove(remove_env_cmd) => remove_env(remove_env_cmd),
                EnvSubCommand::Describe(describe_env_cmd) => describe_env(describe_env_cmd),
            }
        },
        RqSubCommand::Copy(mut migrate_cmd) => {
            let source_infos = load_env_parameters_or_from_connection_string(migrate_cmd.source_env.clone(), &String::new());
            let mut source_con = connect(&source_infos).connection;
            let target_infos = load_env_parameters_or_from_connection_string(migrate_cmd.target_env.clone(), &String::new());
            let mut target_con = connect(&target_infos).connection;
            migrate(&mut migrate_cmd, &mut source_con, &mut target_con);
        },
        RqSubCommand::Connect(connect_cmd) => {
            let connection_infos = load_env_parameters_or_from_connection_string(connect_cmd.env.clone(), &connect_cmd.connection_string);
            let mut connection = connect(&connection_infos);
            shell(&mut connection.connection, connection.host, connection.port, connection.db);
        }
    }
}