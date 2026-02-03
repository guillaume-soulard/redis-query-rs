mod parameters;

extern crate core;

use redis::Connection;
use crate::parameters::{parse_parameters, ExecSubCommand, RqParameters, RqSubCommand};

fn main() {
    let parameters: RqParameters = parse_parameters();
    match parameters.command {
        RqSubCommand::Exec(cmd) => {
            let mut con = connect(&cmd);
            exec_command(&mut con, cmd);
        }
    }
    // execute(&mut con, vec!["SET", "foo", "bar"]);
    // execute(&mut con, vec!["GET", "foo"]);
}

fn exec_command(connection: &mut Connection, exec_command: ExecSubCommand) {
    let commands = exec_command.command.split(' ').collect::<Vec<&str>>();
    execute(connection, commands);
}

fn execute(con: &mut Connection, command: Vec<&str>) {
    if command.len() == 0 {
        return;
    }
    let name = command[0];
    let mut cmd = &mut redis::cmd(name);
    for &c in command.iter().skip(1) {
        cmd = cmd.arg(c)
    }
    match cmd.query::<String>(con) {
        Ok(r) => println!("{}", r),
        Err(e) => println!("Error: {}", e),
    }
}
fn connect(command: &ExecSubCommand) -> Connection {
    let mut user_and_password: String = String::new();
    if command.user != "" {
        user_and_password.push_str(command.user.as_str());
    }
    if command.password != "" {
        user_and_password.push_str(":");
        user_and_password.push_str(command.password.as_str());
    }
    if user_and_password != "" {
        user_and_password.push_str("@");
    }
    let client = redis::Client::open(
        format!("redis://{}{}:{}/{}",
                user_and_password,
                command.host,
                command.port,
                command.db
        )
    );
    match client {
        Ok(c) => match c.get_connection() {
            Ok(c) => c,
            Err(e) => panic!("Error: {}", e),
        },
        Err(e) => panic!("{}", e),
    }
}
