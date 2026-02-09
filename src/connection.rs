use std::process::exit;
use crate::parameters::Connectable;
use redis::Connection;
use crate::io::writeln_to_stderr;

pub fn connect(command: &dyn Connectable) -> Connection {
    if command.get_sentinel_addrs() != "" {
        connect_via_sentinel(command)
    } else {
        direct_connect_to_node(command)
    }
}

fn direct_connect_to_node(command: &dyn Connectable) -> Connection {
    connect_to_redis(
        command.get_user(),
        command.get_password(),
        command.get_host(),
        command.get_port(),
        command.get_db(),
    )
}

fn connect_to_redis(user: String, password: String, host: String, port: u16, db: u8) -> Connection {
    let user_and_password = get_user_and_password_connection_string(user, password);
    let client = match redis::Client::open(format!(
        "redis://{}{}:{}/{}",
        user_and_password, host, port, db
    )) {
        Ok(c) => c,
        Err(e) => {
            writeln_to_stderr(format!("Cannot connect to redis instance {}:{} -> {}", host, port, e.to_string()).to_string());
            exit(1);
        },
    };
    match client.get_connection() {
        Ok(c) => c,
        Err(e) => {
            writeln_to_stderr(format!("Cannot connect to redis instance {}:{} -> {}", host, port, e.to_string()).to_string());
            exit(1);
        }
    }
}

fn get_user_and_password_connection_string(user: String, password: String) -> String {
    let mut user_and_password: String = String::new();
    if user != "" {
        user_and_password.push_str(user.as_str());
    }
    if password != "" {
        user_and_password.push_str(":");
        user_and_password.push_str(password.as_str());
    }
    if user_and_password != "" {
        user_and_password.push_str("@");
    }
    user_and_password
}

fn connect_via_sentinel(command: &dyn Connectable) -> Connection {
    for sentinel_addrs in command.get_sentinel_addrs().split(',') {
        let sentinel_host_and_port = sentinel_addrs.split_once(':').unwrap();
        let client = match redis::Client::open(format!(
            "redis://{}:{}",
            sentinel_host_and_port.0, sentinel_host_and_port.1
        )) {
            Ok(connection) => connection,
            Err(e) => {
                writeln_to_stderr(format!("Cannot connect to sentinel instance {}:{} -> {}",sentinel_host_and_port.0, sentinel_host_and_port.1, e.to_string()).to_string());
                exit(1);
            },
        };
        let mut sentinel_connection = match client.get_connection() {
            Ok(connection) => connection,
            Err(e) => {
                writeln_to_stderr(format!("Cannot connect to sentinel instance {}:{} -> {}",sentinel_host_and_port.0, sentinel_host_and_port.1, e.to_string()).to_string());
                exit(1);
            }
        };
        let sentinel_response = redis::cmd("SENTINEL")
            .arg("GET-MASTER-ADDR-BY-NAME")
            .arg(command.get_sentinel_master().as_str())
            .query::<Vec<String>>(&mut sentinel_connection);
        let r = match sentinel_response {
            Ok(r) => r,
            Err(e) => {
                writeln_to_stderr(format!("Cannot get sentinel master address: {}", e).to_string());
                exit(1);
            }
        };
        return connect_to_redis(
            command.get_user(),
            command.get_password(),
            r[0].clone(),
            r[1].parse().unwrap(),
            command.get_db(),
        );
    }
    writeln_to_stderr("No sentinel instance available".to_string());
    exit(1);
}
