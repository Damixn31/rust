use crate::clock::{notification::notify, timer::tempo};

pub fn pomodoro_run(w_duration: u64, s_break: u64, l_break: u64, cycles: u64) {
    for i in 1..=cycles {
        println!(
            "\tPomodoro {}: Trabajando durante {} minutos.",
            i, w_duration
        );
        tempo(w_duration);
        notify("Pomodoro Completado toma un descanso de 15 minutos.");
        //println!(
        //    "Pomodoro {} completado! Toma un descanso corto de {} minutos.",
        //    i, s_break
        //);
        tempo(s_break);
    }
    println!(
        "Ciclo completado Toma un descanso largo de {} minutos",
        l_break
    );
    tempo(l_break);
    println!("Sesion de pomodoro completada!");
}
