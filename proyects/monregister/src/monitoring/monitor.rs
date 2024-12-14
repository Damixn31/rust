use super::regex_util::compile_regex;
use colored::*;
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
            println!(
                "{}",
                "\n\t\tMonitoreando intentos de ingresos".green().bold()
            );
            println!(
                "{}",
                "\t---------------------------------------------------"
                    .white()
                    .bold()
            );
            print_summary(ip_attempts, max_attempt);

            println!(
                "{}",
                format!("\n\tIntentos de ingresos fallidos desde IP: {}", ip)
                    .red()
                    .bold()
            );

            if ip_attempts[&ip] > max_attempt {
                println!("Alerta! Demasiados intentos fallidos desde IP: {}", ip);
            }
        }
    }
}

fn print_summary(ip_attempts: &HashMap<String, u32>, max_attempt: u32) {
    println!("{}", "\n\tResumen de intentos de ingresos fallidos:".blue());
    for (ip, count) in ip_attempts {
        println!(
            "\n\t 🔸 IP:  {} - Intentos : {}",
            ip.magenta().bold(),
            count
        );
    }

    for (ip, count) in ip_attempts {
        if *count > max_attempt {
            println!("Demasiados intentos fallidos desde IP: {}", ip);
        }
    }
}
