use std::collections::HashMap;

use device_query::Keycode;

pub fn get_key_mapping(key: Keycode) -> Option<&'static str> {
    let key_map = build_key_map();
    key_map.get(&key).copied() // para devolver una copia de &str
}

fn build_key_map() -> HashMap<Keycode, &'static str> {
    let mut key_map: HashMap<Keycode, &'static str> = HashMap::new();

    key_map.insert(Keycode::A, "a");
    key_map.insert(Keycode::B, "b");
    key_map.insert(Keycode::C, "c");
    key_map.insert(Keycode::D, "d");
    key_map.insert(Keycode::E, "e");
    key_map.insert(Keycode::F, "f");
    key_map.insert(Keycode::G, "g");
    key_map.insert(Keycode::H, "h");
    key_map.insert(Keycode::I, "i");
    key_map.insert(Keycode::J, "j");
    key_map.insert(Keycode::K, "k");
    key_map.insert(Keycode::L, "l");
    key_map.insert(Keycode::M, "m");
    key_map.insert(Keycode::N, "n");
    key_map.insert(Keycode::Grave, "ñ");
    key_map.insert(Keycode::O, "o");
    key_map.insert(Keycode::P, "p");
    key_map.insert(Keycode::Q, "q");
    key_map.insert(Keycode::R, "r");
    key_map.insert(Keycode::S, "s");
    key_map.insert(Keycode::T, "t");
    key_map.insert(Keycode::U, "u");
    key_map.insert(Keycode::V, "v");
    key_map.insert(Keycode::W, "w");
    key_map.insert(Keycode::X, "x");
    key_map.insert(Keycode::Y, "y");
    key_map.insert(Keycode::Z, "z");

    // numeros
    key_map.insert(Keycode::Key0, "0");
    key_map.insert(Keycode::Key1, "1");
    key_map.insert(Keycode::Key2, "2");
    key_map.insert(Keycode::Key3, "3");
    key_map.insert(Keycode::Key4, "4");
    key_map.insert(Keycode::Key5, "5");
    key_map.insert(Keycode::Key6, "6");
    key_map.insert(Keycode::Key7, "7");
    key_map.insert(Keycode::Key8, "8");
    key_map.insert(Keycode::Key9, "9");

    // F
    key_map.insert(Keycode::F1, "[F1]");
    key_map.insert(Keycode::F2, "[F2]");
    key_map.insert(Keycode::F3, "[F3]");
    key_map.insert(Keycode::F4, "[F4]");
    key_map.insert(Keycode::F5, "[F5]");
    key_map.insert(Keycode::F6, "[F6]");
    key_map.insert(Keycode::F7, "[F7]");
    key_map.insert(Keycode::F8, "[F8]");
    key_map.insert(Keycode::F9, "[F9]");
    key_map.insert(Keycode::F10, "[F10]");
    key_map.insert(Keycode::F11, "[F11]");
    key_map.insert(Keycode::F12, "[F12]");

    // teclas especiales
    key_map.insert(Keycode::Space, " ");
    key_map.insert(Keycode::Enter, "[ENTER]");
    key_map.insert(Keycode::Tab, "[TAB]");
    key_map.insert(Keycode::Backspace, "[BACKSPACE]");
    key_map.insert(Keycode::Escape, "[ESC]");
    key_map.insert(Keycode::LShift, "[SHIFT]");
    key_map.insert(Keycode::RShift, "[SHIFT]");
    key_map.insert(Keycode::LControl, "[CTRL]");
    key_map.insert(Keycode::RControl, "[CTRL]");
    key_map.insert(Keycode::LAlt, "[ALT]");
    key_map.insert(Keycode::RAlt, "[ALT]");
    key_map.insert(Keycode::CapsLock, "[CAPSLOCK]");

    key_map.insert(Keycode::Home, "[HOME]");
    key_map.insert(Keycode::Delete, "[DELETE]");
    key_map.insert(Keycode::Insert, "[INSERT]");
    key_map.insert(Keycode::PageUp, "[PAGEUP]");
    key_map.insert(Keycode::PageDown, "[PAGEDOWN]");
    key_map.insert(Keycode::End, "[END]");

    // flechas
    key_map.insert(Keycode::Up, "[ARRIBA]");
    key_map.insert(Keycode::Down, "[ABAJO]");
    key_map.insert(Keycode::Right, "[DERECHA]");
    key_map.insert(Keycode::Left, "[IZQUIERDA]");

    key_map.insert(Keycode::LMeta, "[WIN]");
    key_map.insert(Keycode::RMeta, "[RMETA]");

    // operadores
    key_map.insert(Keycode::Minus, "-");
    key_map.insert(Keycode::Equal, "=");
    key_map.insert(Keycode::LeftBracket, "[");
    key_map.insert(Keycode::RightBracket, "]");
    key_map.insert(Keycode::BackSlash, "\\");
    key_map.insert(Keycode::Semicolon, ";");
    key_map.insert(Keycode::Apostrophe, "'");
    key_map.insert(Keycode::Comma, ",");
    key_map.insert(Keycode::Dot, ".");
    key_map.insert(Keycode::Slash, "/");

    key_map
}
