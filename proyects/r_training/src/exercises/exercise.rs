#[derive(Debug)]
pub struct Exercise {
    pub name: String,
    pub duration_secs: Option<u32>,
    pub audio_file: Option<String>,
    pub completion_sound: Option<String>,
    pub completed: bool,
}

impl Exercise {
    pub fn new(
        name: &str,
        duration_secs: Option<u32>,
        audio_file: Option<String>,
        completion_sound: Option<String>,
        completed: bool,
    ) -> Self {
        Exercise {
            name: name.to_string(),
            duration_secs,
            audio_file: audio_file.map(|s| s.to_string()),
            completion_sound: completion_sound.map(|s| s.to_string()),
            completed: false,
        }
    }
}
