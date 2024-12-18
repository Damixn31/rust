use std::process::Command;

use super::regex_util::compile_regex_connect_ips;

pub fn get_connect_ips() -> Vec<String> {
    let output = Command::new("ss")
        .arg("-ntu")
        .output()
        .expect("Failed to execute ss command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut ips = Vec::new();

    let re = compile_regex_connect_ips().unwrap();
    for line in output_str.lines() {
        if let Some(captures) = re.captures(line) {
            let ip = captures.get(1).unwrap().as_str().to_string();
            if !ips.contains(&ip) {
                ips.push(ip);
            }
        }
    }

    ips
}
