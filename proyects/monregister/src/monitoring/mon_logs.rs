use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
};

use super::{
    journalctl::spawn_journalctl_process,
    regex_util::{compile_regex_mariadb, compile_regex_sshd},
};

pub fn monitor_logs(
    service: &str,
    ip_attempts: Arc<Mutex<HashMap<String, u32>>>,
    max_attempt: u32,
) {
    let re = match service {
        "mariadb.service" => compile_regex_mariadb().unwrap(),
        "sshd.service" => compile_regex_sshd().unwrap(),
        _ => {
            eprintln!("No regex defined for service: {}", service);
            return;
        }
    };
    if let Ok(mut process) = spawn_journalctl_process(service) {
        let stdout = process.stdout.take().expect("Failed to capture stdout");
        let reader = BufReader::new(stdout);

        println!("Monitoreando logs para el servicio: {}", service);

        for log in reader.lines().map_while(Result::ok) {
            if let Some(captures) = re.captures(&log) {
                match service {
                    "mariadb.service" => {
                        let user = &captures[1];
                        let host = &captures[2];
                        println!(
                            "Acceso denegado para el usuario '{}' desde el host '{}",
                            user, host
                        );
                    }
                    "sshd.service" => {
                        let ip = &captures[1];
                        println!("Intento fallido de conexion desde IP: {}", ip);

                        // Actualizacion de contador
                        let mut attempts = ip_attempts.lock().unwrap();
                        let counter = attempts.entry(ip.to_string()).or_insert(0);
                        *counter += 1;

                        if *counter >= max_attempt {
                            println!("Alerta: Demasiados intentos fallidos desde la IP: {}", ip);
                        }
                    }
                    _ => {}
                }
            }
        }
        if let Err(e) = process.wait() {
            eprintln!("Error al responder el proceso journalctl: {}", e);
        }
    } else {
        eprintln!(
            "No se pudo iniciar journalctl para el servicio: {}",
            service
        );
    }
}
