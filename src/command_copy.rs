use crate::io::writeln_to_stderr;
use crate::parameters::MigrateSubCommand;
use redis::{Connection, Iter, RedisResult, ScanOptions, TypedCommands};
use std::process::exit;

pub fn migrate(
    migrate_sub_command: &mut MigrateSubCommand,
    source_con: &mut Connection,
    target_con: &mut Connection,
) {
    let scan_opt = ScanOptions::default()
        .with_pattern(migrate_sub_command.source_pattern.clone())
        .with_count(migrate_sub_command.count);
    let result: RedisResult<Iter<String>> = source_con.scan_options(scan_opt);
    match result {
        Ok(iter) => {
            let keys: Vec<String> = iter
                .take(migrate_sub_command.limit)
                .map(|k| k.unwrap())
                .collect();
            for key in keys {
                let dumped_key = dump_key_on_source(migrate_sub_command, key.clone(), source_con);
                restore_key_on_target(
                    migrate_sub_command,
                    dumped_key.0,
                    dumped_key.1,
                    key,
                    target_con,
                );
            }
        }
        Err(err) => {
            writeln_to_stderr(format!(
                "Error while scanning keys from source: {}",
                err.to_string()
            ));
            exit(1);
        }
    }
}

fn dump_key_on_source(
    migrate_sub_command: &MigrateSubCommand,
    key: String,
    source_con: &mut Connection,
) -> (Vec<u8>, i64) {
    let ttl: i64;
    if migrate_sub_command.ttl == -3 {
        let result = redis::cmd("TTL").arg(key.clone()).query(source_con);
        ttl = match result {
            Ok(t) => t,
            Err(err) => {
                writeln_to_stderr(format!(
                    "Error while dumping key TTL {} : {}",
                    key,
                    err.to_string()
                ));
                exit(1);
            }
        }
    } else {
        ttl = migrate_sub_command.ttl;
    }
    let result= redis::cmd("DUMP").arg(key.clone()).query(source_con);
    let dumped_value = match result {
        Ok(d) => d,
        Err(err) => {
            writeln_to_stderr(format!(
                "Error while dumping key {} : {}",
                key,
                err.to_string()
            ));
            exit(1);
        }
    };
    (dumped_value, ttl)
}

fn restore_key_on_target(
    migrate_sub_command: &MigrateSubCommand,
    dumped_value: Vec<u8>,
    ttl: i64,
    key: String,
    target_con: &mut Connection,
) {
    let restore_command = &mut redis::cmd("RESTORE");
    let final_ttl = if ttl < 0 { 0 } else { ttl };
    restore_command
        .arg(key.clone())
        .arg(final_ttl)
        .arg(dumped_value);
    if migrate_sub_command.replace {
        restore_command.arg("REPLACE");
    }
    match restore_command.query::<String>(target_con) {
        Ok(_) => {}
        Err(err) => {
            writeln_to_stderr(format!(
                "Error while restoring key {} with ttl {} : {}",
                key,
                final_ttl,
                err.to_string()
            ));
            exit(1);
        }
    };
}
