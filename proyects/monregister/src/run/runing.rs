use std::collections::HashMap;

use crate::monitoring::{
    journalctl::{get_stdout_reader, spawn_journalctl_process},
    monitor::monitor_failed_login_attempts,
};

pub fn run() {
    let max_attempt = 5;

    let mut ip_attempts = HashMap::new();

    if let Ok(process) = spawn_journalctl_process() {
        let mut reader = get_stdout_reader(process).expect("Failed to get stdout reader");

        monitor_failed_login_attempts(&mut reader, &mut ip_attempts, max_attempt);
        //monitor::print_summary(&ip_attempts, max_attempt);
    } else {
        eprintln!("Failed to spawn journalctl process");
    }
}

