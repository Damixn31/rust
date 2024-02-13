//use crate::load_exercises::load_weekly_exercises;

use std::{
    env,
    io::{self, BufRead},
};

use crate::{
    audio::play_audio::play_audio, load_exercises::load_weekly_exercises::load_weekly_exercises,
    timing::run_timer::timmer,
};

pub fn executable() {
    let weekly_exercises = load_weekly_exercises();

    if let Some(first_monday) = weekly_exercises.get("Lunes") {
        println!("Ejercicios del dia Lunes:");
        for exercise in first_monday {
            println!("Comenzando ejercicios: {}", exercise.name);
            if let Some(duration) = exercise.duration_secs {
                timmer(duration, &exercise.audio_file);
                if let Some(ping) = &exercise.completion_sound {
                    play_audio(ping);
                }
                // aca tengo que poner todas las situaciones tengo pensado usar el match
                //if exercise.name == "Flexiones x12" {
                //    timmer(duration, &exercise.audio_file)
                //}
            } else {
                println!("Presionar Enter para pasar al siguiente ejercicio...");
                let mut input = String::new();
                io::stdin().lock().read_line(&mut input).unwrap();
            }
            println!("Ejercicio {} completado", exercise.name);
        }
    }
}
