extern crate core;
mod parameters;
mod connection;
mod io;
mod command_migrate;
mod command_scan;
mod command_exec;
mod command_env;

use crate::command_exec::exec_command;
use crate::command_migrate::migrate;
use crate::command_scan::scan_command;
use crate::connection::connect;
use crate::command_env::{describe_env, list_env, load_env_parameters, remove_env, set_env};
use crate::parameters::{load_parameters, EnvSubCommand, RqParameters, RqSubCommand};

fn main() {
    let parameters: RqParameters = load_parameters();
    match parameters.command {
        RqSubCommand::Exec(mut cmd) => {
            load_env_parameters(cmd.env.clone(), &mut cmd);
            let mut con = connect(&cmd);
            exec_command(&mut con, cmd);
        }
        RqSubCommand::Scan(mut scan) => {
            load_env_parameters(scan.env.clone(), &mut scan);
            let mut con = connect(&scan);
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
        RqSubCommand::Migrate(mut migrate_cmd) => {
            load_env_parameters(migrate_cmd.source_env.clone(), &mut migrate_cmd);
            let mut source_con = connect(&migrate_cmd);
            load_env_parameters(migrate_cmd.target_env.clone(), &mut migrate_cmd);
            let mut target_con = connect(&migrate_cmd);
            migrate(&mut migrate_cmd, &mut source_con, &mut target_con);
        }
    }
}