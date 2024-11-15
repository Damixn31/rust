pub fn cifrado_cesar(text: &str, key: i32) -> String {
    let mut text_cypher = String::new();

    for word in text.chars() {
        if word.is_alphabetic() {
            let base = if word.is_uppercase() { b'A' } else { b'a' };
            let despl = ((word as u8 - base + (key as u8) % 26) % 26) + base;
            text_cypher.push(despl as char);
        } else {
            text_cypher.push(word) // no cifra caracteres especiales
        }
    }

    text_cypher
}
