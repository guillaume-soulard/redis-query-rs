use std::process::exit;
use crate::parameters::Connectable;
use redis::Connection;
use regex::Regex;
use crate::io::writeln_to_stderr;

pub fn connect(command: &dyn Connectable) -> (Connection, String, u16, u8) {
    if command.get_sentinel_addrs() != "" {
        connect_via_sentinel(command)
    } else {
        direct_connect_to_node(command)
    }
}

enum RedisConnectionKind {
    STANDALONE,
    SENTINEL,
    REPLICAS,
    CLUSTER
}

fn get_redis_connection(command: &dyn Connectable) -> RedisConnectionKind {
    let regex = Regex::new("(?<connectionType>redis):\\/\\/(?:(?<userName>[^:]*)?(?::(?<password>[^@]*)?@))?(?<host>[^:]*)(?::(?<port>[1-9][0-9]*))?(?:\\/(?<db>[0-9]*))?(?:\\?protocol=(?<protocol>resp(?:[23])))?").unwrap();
    match regex.captures(command.get_instance().as_str()) {
        Some(captures) => {
            let connection_type = &captures["connectionType"];
            match connection_type {
                "redis" => RedisConnectionKind::STANDALONE,
                "sentinel" => RedisConnectionKind::SENTINEL,
                "cluster" => RedisConnectionKind::CLUSTER,
                "replica" => RedisConnectionKind::REPLICAS,
                _ => {
                    writeln_to_stderr(format!("Unsupported connection type {}", connection_type).to_string());
                    exit(1);
                }
            }
        },
        None => {
            writeln_to_stderr("unable to parse instance string".to_string());
            exit(1);
        }
    }
}

fn direct_connect_to_node(command: &dyn Connectable) -> (Connection, String, u16, u8) {
    connect_to_redis(
        command.get_user(),
        command.get_password(),
        command.get_host(),
        command.get_port(),
        command.get_db(),
    )
}

fn connect_to_redis(user: String, password: String, host: String, port: u16, db: u8) -> (Connection, String, u16, u8) {
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
        Ok(c) => {
            (c, host, port, db)
        },
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

fn connect_via_sentinel(command: &dyn Connectable) -> (Connection, String, u16, u8) {
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
