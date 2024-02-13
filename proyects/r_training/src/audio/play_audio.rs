use std::process::{Command, Stdio};

pub fn play_audio(file: &str) {
    let _ = Command::new("mpg123")
        .arg(file)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Error al reproducir fichero de audio");
}
