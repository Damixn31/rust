use std::collections::HashMap;

use crate::{audio::get_audio::get_audio_path, exercises::exercise::Exercise};

pub fn load_weekly_exercises() -> HashMap<&'static str, Vec<Exercise>> {
    let mut weekly_exercises = HashMap::new();

    let audio_ping = get_audio_path("AUDIO_PATH_PING", "Error al encontrar ruta audio ping");

    let audio_path_jump =
        get_audio_path("AUDIO_PATH_JUMP", "Error al encontrar la ruta audio salto");

    let audio_path_flex_x12 = get_audio_path(
        "AUDIO_PATH_FLEX",
        "Error al encontrar la ruta audio de flexiones x12",
    );

    let audio_apoyo_de_brazo_x14 = get_audio_path(
        "AUDIO_FLEX_APOYO_DE_BRAZO_14",
        "Error al encontrar ruta de audio flexiones apoyo de brazo x14",
    );

    let audio_sentadillas_con_cambio_de_pierna_x12 = get_audio_path(
        "AUDIO_SENTADILLA_CON_CAMBIO_DE_PIERNA_12",
        "Error al encontrar la ruta de sentadilla",
    );

    let audio_fexiones_offset_x14 = get_audio_path(
        "AUDIO_FLEXIONES_OFFSET_14",
        "Error al encontrar la ruta audio flexiones offset x14",
    );

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
            Some(audio_path_flex_x12),
            Some(audio_ping.clone()),
            false,
        ),
        Exercise::new(
            "Flexiones con inclinacion (apoyo de brazo) x14",
            None,
            Some(audio_apoyo_de_brazo_x14),
            Some(audio_ping.clone()),
            false,
        ),
        Exercise::new(
            "Sentadilla con cambio de piernas x12",
            None,
            Some(audio_sentadillas_con_cambio_de_pierna_x12),
            Some(audio_ping.clone()),
            false,
        ),
        Exercise::new(
            "Flexiones offset x14",
            None,
            Some(audio_fexiones_offset_x14),
            Some(audio_ping.clone()),
            false,
        ),
    ];

    weekly_exercises.insert("Lunes", first_monday);

    weekly_exercises
}
