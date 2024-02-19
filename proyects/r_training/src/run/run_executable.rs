use std::io::{self, BufRead};

use crate::{
    audio::play_audio::play_audio, load_exercises::load_weekly_exercises::load_weekly_exercises,
    timing::run_timer::timmer,
};

pub fn executable() {
    let weekly_exercises = load_weekly_exercises();
    for (index, exercise) in weekly_exercises["Lunes"].iter().enumerate() {
        if let Some(audio_file) = &exercise.audio_file {
            play_audio(audio_file);
        }

        println!("\tComenzando ejercicio: {}", exercise.name);

        // Ejecucion de temporizador si tiene una duracion especifica
        if let Some(duration) = exercise.duration_secs {
            timmer(duration);

            if let Some(completion_sound) = &exercise.completion_sound {
                play_audio(completion_sound);
            }
            //if index < weekly_exercises["Lunes"].len() - 1 {
            //    if let Some(next_exercise) = &weekly_exercises["Lunes"][index + 1].audio_file {
            //        play_audio(next_exercise);
            //    }
            //}
        } else {
            println!("\tPresionar Enter para pasar al siguiente ejercicio...");
            let mut input = String::new();
            io::stdin().lock().read_line(&mut input).unwrap();
        }
        println!("\tEjercicio {} completado", exercise.name);

        if index < weekly_exercises["Lunes"].len() - 1 {
            let rest_duration = 1;
            println!("Descansa durante {} segundos...", rest_duration);
            timmer(rest_duration);
        }
    }
}
