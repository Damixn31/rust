use std::{collections::HashMap, env};

use crate::{audio::get_audio::get_audio_path, exercises::exercise::Exercise};

pub fn load_weekly_exercises() -> HashMap<&'static str, Vec<Exercise>> {
    let mut weekly_exercises = HashMap::new();

    let audio_ping = get_audio_path("AUDIO_PATH_PING", "Error al encontrar ruta");
    //let audio_ping = match env::var("AUDIO_PATH_PING") {
    //    Ok(path) => path,
    //    Err(_) => "Error al encontrar la ruta".to_string(),
    //};

    let audio_path_jump = match env::var("AUDIO_PATH_JUMP") {
        Ok(path) => path,
        Err(_) => "Error en la ruta".to_string(),
    };

    let audio_path_flex = match env::var("AUDIO_PATH_FLEX") {
        Ok(path) => path,
        Err(_) => "Error en la ruta".to_string(),
    };
    let audio_apoyo_de_brazo_14 = match env::var("AUDIO_FLEX_APOYO_DE_BRAZO_14") {
        Ok(path) => path,
        Err(_) => "Error en la ruta".to_string(),
    };

    let first_monday = vec![
        Exercise::new(
            "Salto de tijera",
            Some(5),
            Some(audio_path_jump),
            Some(audio_ping.clone()),
            false,
        ),
        Exercise::new(
            "Flexiones x12",
            None,
            Some(audio_path_flex),
            Some(audio_ping.clone()),
            false,
        ),
        Exercise::new(
            "Flexiones con inclinacion (apoyo de brazo) x14",
            None,
            Some(audio_apoyo_de_brazo_14),
            Some(audio_ping.clone()),
            false,
        ),
    ];

    weekly_exercises.insert("Lunes", first_monday);

    weekly_exercises
}
