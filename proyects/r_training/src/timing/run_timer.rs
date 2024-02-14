use std::{thread, time::Duration};

pub fn timmer(duration_secs: u32) {
    //let ping_song = env::var("PATH_FILE").expect("Ruta de fichero no funciona");
    println!("\r\tInicio de temporizador de {} segundos!", duration_secs);
    for i in (1..=duration_secs).rev() {
        // aca quiero definir un temporizador que sea lindo a la vista verse en la consola
        print!("\r\tTiempo restante: {} segundos", i);
        thread::sleep(Duration::from_secs(1));
    }
}
