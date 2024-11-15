use super::cifrado::cifrado_cesar;

pub fn desifrate_cesar(text: &str, key: i32) -> String {
    cifrado_cesar(text, -key)
}
