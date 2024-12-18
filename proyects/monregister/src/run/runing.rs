use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use crate::monitoring::mon_logs::monitor_logs;

pub fn run() {
    let services = vec!["sshd.service", "mariadb.service"];

    let max_attempt = 1;

    let ip_attempts = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    for service in services {
        let ip_attempts = Arc::clone(&ip_attempts);
        let handle = thread::spawn(move || {
            monitor_logs(service, ip_attempts, max_attempt);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
