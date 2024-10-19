use std::collections::HashMap;

//Contar Frecuencia de Palabras
//Dada una cadena de texto, implementa una función que cuente la frecuencia de cada palabra usando un HashMap<String, i32>.
pub fn word_frequency(word: &str) -> HashMap<String, i32> {
    let mut frequency = HashMap::new();
    for p in word.split_whitespace() {
        let p = p.to_lowercase();
        *frequency.entry(p).or_insert(0) += 1;
    }
    frequency
}
