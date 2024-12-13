use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

fn main() {
    let max_attempt = 5;

    let mut ip_attempts: HashMap<String, u32> = HashMap::new();

    let process = Command::new("journalctl")
        .arg("-f")
        .arg("_SYSTEMD_UNIT=sshd.service")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute journalctl -f");

    //let logs = String::from_utf8_lossy(&output.stdout);
    let stdout = process.stdout.expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    let re = regex::Regex::new(r"Failed password for .* from (\d+\.\d+\.\d+\.\d+) port").unwrap();

    println!("\nMonitoreando intentos fallidos");
    println!("----------------------------------");
    for line in reader.lines() {
        if let Ok(log) = line {
            if let Some(captures) = re.captures(&log) {
                let ip = captures.get(1).unwrap().as_str().to_string();
                *ip_attempts.entry(ip.clone()).or_insert(0) += 1;
                println!("Intentos de ingresos fallidos desde IP: {}", ip);

                if ip_attempts[&ip] > max_attempt {
                    println!("Alerta! Demasiados intentos fallidos desde IP: {}", ip);
                }
            }
        }
    }

    println!("Resumen de intentos de ingresos fallidos:");
    for (ip, count) in &ip_attempts {
        println!("IP: {} - Intentos: {}", ip, count);
    }

    for (ip, count) in &ip_attempts {
        if *count > max_attempt {
            println!("Demasiados intentos fallidos desde IP: {}", ip);
        }
    }
}
