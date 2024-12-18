use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::ChildStdout,
};

use crate::notifys::notify_local::send_notification;

use super::regex_util::compile_regex_sshd;

pub fn monitor_sshd_logs(
    reader: BufReader<ChildStdout>,
    ip_attempts: &mut HashMap<String, u32>,
    max_attempts: u32,
) {
    let re = compile_regex_sshd().unwrap();

    for line in reader.lines().map_while(Result::ok) {
        if let Some(captures) = re.captures(&line) {
            let ip = captures[1].to_string();
            let attempts = ip_attempts.entry(ip.clone()).or_insert(0);
            *attempts += 1;

            println!("Intenttos de ingresos fallidos desde IP: {}", ip);

            if *attempts > max_attempts {
                println!("Alerta! Demasiados intentos fallidos desde IP: {}", ip);
                send_notification(&format!("Demasiados intentos fallidos desde la IP: {}", ip));
            }
        }
    }
}
