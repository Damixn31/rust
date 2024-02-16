use std::env;

pub fn get_audio_path(variable_name: &str, default_message: &str) -> String {
    match env::var(variable_name) {
        Ok(path) => path,
        Err(_) => default_message.to_string(),
    }
}
