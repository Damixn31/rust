use std::env;
use std::process::{Command, Stdio};
use std::{thread, time::Duration};

use crate::audio::play_audio::play_audio;

pub fn timmer(duration_secs: u32, audio_file: &Option<String>) {
    //let ping_song = env::var("PATH_FILE").expect("Ruta de fichero no funciona");
    println!("\r\tInicio de temporizador de {} segundos!", duration_secs);
    for i in (1..=duration_secs).rev() {
        // aca quiero definir un temporizador que sea lindo a la vista verse en la consola
        print!("\rTiempo restante: {} segundos", i);
        thread::sleep(Duration::from_secs(1));
    }
    println!("\rTiempo terminado!");

    if let Some(file) = audio_file {
        play_audio(file);
    }

    //let _ = Command::new("mpg123")
    //    .arg(path_file)
    //    .stdout(Stdio::null())
    //    .stderr(Stdio::null())
    //    .status();
}
