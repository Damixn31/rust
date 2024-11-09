use colored::*;
use core::str;
use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("\n\tUso: {} <IP>", args[0]);
        std::process::exit(1);
    }

    let ip = &args[1];

    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip)
        .output()
        .expect("Error al ejecutar el comando ping");

    let output_str =
        str::from_utf8(&output.stdout).expect("\n\t [❗] Error al convertir la salida a texto");

    if let Some(ttl) = output_str.lines().find_map(|line| {
        if line.contains("ttl=") {
            line.split_whitespace()
                .find(|&part| part.starts_with("ttl="))
                .and_then(|ttl_part| ttl_part[4..].parse::<u8>().ok())
        } else {
            None
        }
    }) {
        let system_type = identify_system_by_ttl(ttl);
        print!(
            "\n\t 🔸 {}: {} | {}: {} | {} => {}\n",
            "IP".white().bold(),
            ip.black(),
            "ttl".magenta().bold(),
            ttl,
            "Sistema".bright_yellow(),
            system_type.yellow()
        );
    } else {
        eprintln!(
            "{}",
            "\n\t [❗] No se pudo encontrar el valor del TTL en la salida del ping".red()
        );
    }
}

fn identify_system_by_ttl(ttl: u8) -> &'static str {
    match ttl {
        0..=64 => "Linux / MacOS / BSD",
        65..=128 => "Woindows",
        129..=254 => "Router (Cisco)",
        _ => "Desconocido",
    }
}
