use crate::monitoring::clean_terminal::clear_and_update_terminal;
use crate::notifys::notify_local::send_notification;

use super::connecting::get_connect_ips;
use super::regex_util::compile_regex;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};

pub fn monitor_failed_login_attempts(
    reader: &mut BufReader<std::fs::File>,
    ip_attempts: &mut HashMap<String, u32>,
    max_attempt: u32,
) {
    for line in reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
    {
        if let Some(captures) = compile_regex().and_then(|re| re.captures(&line)) {
            let ip = captures.get(1).unwrap().as_str().to_string();
            *ip_attempts.entry(ip.clone()).or_insert(0) += 1;

            let connectd_ips = get_connect_ips();

            clear_and_update_terminal(ip_attempts, &connectd_ips);

            if ip_attempts[&ip] > max_attempt {
                println!("\tAlerta! Demasiados intentos fallidos desde IP: {}", ip);
                send_notification(&ip, ip_attempts[&ip]);
            }
        }
    }
}
