use crate::parameters::Connectable;
use redis::Connection;

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
    redis::Client::open(format!(
        "redis://{}{}:{}/{}",
        user_and_password, host, port, db
    ))
    .unwrap_or_else(|e| panic!("{}", e))
    .get_connection()
    .unwrap_or_else(|e| panic!("{}", e))
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
        let mut sentinel_connection = redis::Client::open(format!(
            "redis://{}:{}",
            sentinel_host_and_port.0, sentinel_host_and_port.1
        ))
        .unwrap_or_else(|e| panic!("{}", e))
        .get_connection()
        .unwrap_or_else(|e| panic!("{}", e));
        let sentinel_response = redis::cmd("SENTINEL")
            .arg("GET-MASTER-ADDR-BY-NAME")
            .arg(command.get_sentinel_master().as_str())
            .query::<Vec<String>>(&mut sentinel_connection);
        let r = sentinel_response.unwrap_or_else(|e| panic!("{}", e));
        return connect_to_redis(
            command.get_user(),
            command.get_password(),
            r[0].clone(),
            r[1].parse().unwrap(),
            command.get_db(),
        );
    }
    panic!("No sentinel available");
}
