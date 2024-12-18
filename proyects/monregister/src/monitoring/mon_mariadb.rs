use std::{
    io::{BufRead, BufReader},
    process::ChildStdout,
};

use crate::notifys::notify_local::send_notification;

use super::regex_util::compile_regex_mariadb;

pub fn monitor_mariadb_log(reader: BufReader<ChildStdout>) {
    let re = compile_regex_mariadb().unwrap();

    for line in reader.lines() {
        match line {
            Ok(log) => {
                if let Some(captures) = re.captures(&log) {
                    let user = &captures[1];
                    let host = &captures[2];
                    println!(
                        "Acceso denegado para el usuario: '{}' desde el host '{}'",
                        user, host
                    );
                    send_notification(&format!(
                        "Alerta acceso denegado para el usuario '{}' desde el host '{}'",
                        user, host
                    ));
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}
