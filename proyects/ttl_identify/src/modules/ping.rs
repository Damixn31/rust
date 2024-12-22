use core::str;
use std::process::Command;

pub fn run_ping(ip: &str) -> Result<String, String> {
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip)
        .output()
        .expect("Error al ejecutar el comando ping");

    str::from_utf8(&output.stdout)
        .map(|s| s.to_string())
        .map_err(|_| "\n\t [❗] Error al convertir la salida a texto".to_string())
}
