use std::{io::Write, thread, time::Duration};

pub fn tempo(min: u64) {
    let total_seconds = min * 60;
    for second in 0..total_seconds {
        let remaining_minutes = (total_seconds - second) / 60;
        let remaining_seconds = (total_seconds - second) % 60;
        //println!(
        //    "Tiempo restante: {} minutos y {} segundos",
        //    (total_seconds - second) / 60,
        //    (total_seconds - second) % 60
        //);
        print!(
            "\r\tTiempo restante: {:02} minutos y {:02} segundos.",
            remaining_minutes, remaining_seconds
        );
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
