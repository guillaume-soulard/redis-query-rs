use crate::io::writeln_to_stderr;
use redis::{Connection, Value};
use regex::Regex;
use std::process::exit;


pub const DEFAULT_HOST:&str = "localhost";
pub const DEFAULT_PORT:u16 = 6379;
pub const DEFAULT_SENTINEL_PORT:u16 = 26379;
pub const DEFAULT_DB:u8 = 0;
pub const DEFAULT_USER:&str = "default";
pub const DEFAULT_PASSWORD:&str = "";
pub const DEFAULT_SENTINEL_MASTER:&str = "";
pub const DEFAULT_PROTOCOL:&str = "resp2";

pub struct RedisConnection {
    pub connection: Connection,
    pub host: String,
    pub port: u16,
    pub db: u8,
}

pub fn connect(connection_infos: &RedisConnectionInfos) -> RedisConnection {
    match connection_infos.kind {
        RedisConnectionKind::STANDALONE => standalone_connect(&connection_infos),
        RedisConnectionKind::SENTINEL => sentinel_connect(&connection_infos),
        RedisConnectionKind::REPLICAS => replicas_connect(&connection_infos),
        RedisConnectionKind::CLUSTER => standalone_connect(&connection_infos),
    }
}

pub enum RedisConnectionKind {
    STANDALONE,
    SENTINEL,
    REPLICAS,
    CLUSTER,
}

pub struct RedisConnectionInfos {
    pub kind: RedisConnectionKind,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub db: Option<u8>,
    pub password: Option<String>,
    pub user: Option<String>,
    pub protocol: Option<String>,
    pub sentinel_master: Option<String>,
}

pub fn get_redis_connection_infos(connection_string: &String) -> RedisConnectionInfos {
    let regex = Regex::new("(?<connectionType>(redis|cluster|replica|sentinel))://(?:(?<userName>[^:]*)?(?::(?<password>[^@]*)?@))?(?<host>[^:]*)(?::(?<port>[1-9][0-9]*))?(?:/(?<db>[0-9]*))?(?:\\?(?:protocol=(?<protocol>resp(?:[23])))|(?:sentinelMaster=(?<sentinelMasterName>))|&)?").unwrap();
    match regex.captures(connection_string.as_str()) {
        Some(captures) => {
            let connection_type = &captures["connectionType"];
            let kind = match connection_type {
                "redis" => RedisConnectionKind::STANDALONE,
                "sentinel" => RedisConnectionKind::SENTINEL,
                "cluster" => RedisConnectionKind::CLUSTER,
                "replica" => RedisConnectionKind::REPLICAS,
                _ => {
                    writeln_to_stderr(
                        format!("Unsupported connection type {}", connection_type).to_string(),
                    );
                    exit(1);
                }
            };
            RedisConnectionInfos {
                kind,
                host: captures.name("host").map(|m| m.as_str().to_string()),
                port: captures.name("port").map(|m| m.as_str().parse().unwrap()),
                db: captures.name("db").map(|m| m.as_str().parse().unwrap()),
                password: captures.name("password").map(|m| m.as_str().to_string()),
                user: captures.name("userName").map(|m| m.as_str().to_string()),
                protocol: captures.name("protocol").map(|m| m.as_str().to_string()),
                sentinel_master: captures
                    .name("sentinelMasterName")
                    .map(|m| m.as_str().to_string()),
            }
        }
        None => {
            writeln_to_stderr("unable to parse instance string".to_string());
            exit(1);
        }
    }
}

fn standalone_connect(connection_infos: &RedisConnectionInfos) -> RedisConnection {
    connect_to_instance(connection_infos)
}

fn replicas_connect(connection_infos: &RedisConnectionInfos) -> RedisConnection {
    let con = connect_to_instance(connection_infos);
    let mut connection = con.connection;
    let role_response = match redis::cmd("ROLE").query::<Value>(&mut connection) {
        Ok(Value::Array(v))  => v,
        Ok(v) => {
            writeln_to_stderr(format!("Unexpected value type returned by ROLE : {:?}", v));
            exit(1);
        }
        Err(e) => {
            writeln_to_stderr(format!("Cannot get role of instance {}:{} : {}", con.host, con.port, e).to_string());
            exit(1);
        }
    };
    let role_value = role_response[0].clone();
    let role = match role_value {
        Value::SimpleString(v) => v,
        _ => {
            writeln_to_stderr(format!("Unexpected value type returned by ROLE : {:?}", role_value).to_string());
            exit(1);
        }
    };
    match role.as_str() {
        "master" => {
            RedisConnection {
                connection,
                host: con.host,
                port: con.port,
                db: con.db,
            }
        }
        "slave" => {
            let master_address = match role_response[1].clone() {
                Value::SimpleString(v) => v,
                _ => {
                    writeln_to_stderr(format!("Unexpected value type returned by ROLE : {:?}", role_response[1]).to_string());
                    exit(1);
                }
            };
            let master_port:u16 = match role_response[2].clone() {
                Value::Int(v) => v as u16,
                v => {
                    writeln_to_stderr(format!("Unexpected value type returned by ROLE : {:?}", v).to_string());
                    exit(1);
                }
            };
            let master_connection = connect_to_instance(&RedisConnectionInfos{
                kind: RedisConnectionKind::STANDALONE,
                host: Some(master_address.clone()),
                port: Some(master_port),
                db: connection_infos.db,
                password: connection_infos.password.clone(),
                user: connection_infos.user.clone(),
                protocol: connection_infos.protocol.clone(),
                sentinel_master: connection_infos.sentinel_master.clone(),
            });
            RedisConnection{
                connection: master_connection.connection,
                host: master_address,
                port: master_port,
                db: connection_infos.db.unwrap_or(DEFAULT_DB),
            }
        }
        _ => {
            writeln_to_stderr(format!("Instance {}:{} has an unsupported role: {}", con.host, con.port, role));
            exit(1);
        }
    }
}

fn connect_to_instance(connection_infos: &RedisConnectionInfos) -> RedisConnection {
    let user_and_password = get_user_and_password_connection_string(connection_infos);
    let host = connection_infos
        .host
        .clone()
        .unwrap_or(DEFAULT_HOST.to_string());
    let port = connection_infos.port.clone().unwrap_or(DEFAULT_PORT);
    let db = connection_infos.db.clone().unwrap_or(DEFAULT_DB);
    let protocol = connection_infos.protocol.clone().unwrap_or(DEFAULT_PROTOCOL.to_string());
    let client = match redis::Client::open(format!(
        "redis://{}{}:{}/{}?protocol={}",
        user_and_password, host, port, db, protocol
    )) {
        Ok(c) => c,
        Err(e) => {
            writeln_to_stderr(
                format!(
                    "Cannot connect to redis instance {}:{} -> {}",
                    host,
                    port,
                    e.to_string()
                )
                .to_string(),
            );
            exit(1);
        }
    };
    match client.get_connection() {
        Ok(c) => RedisConnection {
            connection: c,
            host,
            port,
            db,
        },
        Err(e) => {
            writeln_to_stderr(
                format!(
                    "Cannot connect to redis instance {}:{} -> {}",
                    host,
                    port,
                    e.to_string()
                )
                .to_string(),
            );
            exit(1);
        }
    }
}

fn get_user_and_password_connection_string(connection_infos: &RedisConnectionInfos) -> String {
    let mut user_and_password: String = String::new();
    let user = connection_infos
        .user
        .clone()
        .unwrap_or(DEFAULT_USER.to_string());
    let password = connection_infos.password.clone().unwrap_or(DEFAULT_PASSWORD.to_string());
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

fn sentinel_connect(connection_infos: &RedisConnectionInfos) -> RedisConnection {
    let sentinel_host = connection_infos.host.clone().unwrap_or(DEFAULT_HOST.to_string());
    let sentinel_port = connection_infos.port.unwrap_or(DEFAULT_SENTINEL_PORT);
    let client = match redis::Client::open(format!("redis://{}:{}", sentinel_host, sentinel_port)) {
        Ok(connection) => connection,
        Err(e) => {
            writeln_to_stderr(
                format!(
                    "Cannot connect to sentinel instance {}:{} -> {}",
                    sentinel_host,
                    sentinel_port,
                    e.to_string()
                )
                .to_string(),
            );
            exit(1);
        }
    };
    let mut sentinel_connection = match client.get_connection() {
        Ok(connection) => connection,
        Err(e) => {
            writeln_to_stderr(
                format!(
                    "Cannot connect to sentinel instance {}:{} -> {}",
                    sentinel_host,
                    sentinel_port,
                    e.to_string()
                )
                .to_string(),
            );
            exit(1);
        }
    };
    let sentinel_response = redis::cmd("SENTINEL")
        .arg("GET-MASTER-ADDR-BY-NAME")
        .arg(connection_infos.sentinel_master.clone().unwrap_or(DEFAULT_SENTINEL_MASTER.to_string()))
        .query::<Vec<String>>(&mut sentinel_connection);
    let r = match sentinel_response {
        Ok(r) => r,
        Err(e) => {
            writeln_to_stderr(format!("Cannot get sentinel master address: {}", e).to_string());
            exit(1);
        }
    };
    connect_to_instance(&RedisConnectionInfos{
        kind: RedisConnectionKind::STANDALONE,
        host: Some(r[0].clone()),
        port: Some(r[1].clone()).map(|s| s.parse().unwrap()),
        password: connection_infos.password.clone(),
        db: connection_infos.db.clone(),
        sentinel_master: connection_infos.sentinel_master.clone(),
        user: connection_infos.user.clone(),
        protocol: connection_infos.protocol.clone(),
    })
}
