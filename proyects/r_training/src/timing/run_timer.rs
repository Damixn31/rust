use std::process::{Command, Stdio};
use std::{thread, time::Duration};

pub fn timmer(duration_secs: u32) {
    println!("\r\tInicio de temporizador de {} segundos!", duration_secs);
    for i in (1..=duration_secs).rev() {
        println!("\rTiempo restante: {} segundos", i);
        thread::sleep(Duration::from_secs(1));
    }
    println!("\rTiempo terminado!");

    let _ = Command::new("mpg123")
        .arg("/home/2d/Downloads/ping-82822.mp3")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}
