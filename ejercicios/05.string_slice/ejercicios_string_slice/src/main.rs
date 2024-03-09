// 1. Reversión de palabras: Escribe una función que tome una cadena como entrada y devuelva una nueva cadena con las palabras en orden inverso.
// Por ejemplo, si la entrada es "hola mundo", la salida debería ser "mundo hola".
fn word_reverse(input: &str) -> String {
    let mut word: Vec<&str> = input.split_whitespace().collect();
    word.reverse();

    let result: String = word.join(" ");

    result
}

// 2. Contador de palabras: Escribe una función que tome una cadena como entrada y devuelva el número de palabras en esa cadena.
fn count_word(input: &str) -> usize {
    input.split_whitespace().count()
}
// 3. Buscar y reemplazar: Escribe una función que tome una cadena, una palabra a buscar y una palabra de reemplazo,
// y devuelva una nueva cadena con todas las instancias de la palabra buscada reemplazadas por la palabra de reemplazo.
fn search_replacement(input: &str, word: &str, remp: &str) -> String {
    input.replace(word, remp)
}

// 4. Recortar: Escribe una función que tome una cadena y un índice de inicio y fin,
// y devuelva una subcadena que contenga los caracteres entre esos índices.
fn cut_str(input: &str, start_i: usize, end_i: usize) -> Option<&str> {
    if start_i > input.len() || end_i > input.len() || start_i > end_i {
        return None;
    }
    input.get(start_i..end_i)
}

// 5. Eliminar caracteres especiales: Escribe una función que tome una cadena como entrada
// y devuelva una nueva cadena que elimine todos los caracteres que no sean letras o números.
fn delete_char_special(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            result.push(c)
        }
    }
    result
}

fn main() {
    println!("\n---- EJERCICIO 1");
    let word = "Hola como estas";

    let word_inver = word_reverse(word);
    println!("Esta es la palabras invertida: {:?}", word_inver);

    println!("\n---- EJERCICIO 2");
    let c_word = "mi nombre es pepe";

    let res = count_word(c_word);
    println!("las palabras contadas son: {:?}", res);

    println!("\n---- EJERCICIO 3");
    let ej_string = "Esto es un ejemplo para mostrar el hola";
    let word_original = "hola";
    let word_remp = "chau";

    let replacement_string = search_replacement(ej_string, word_original, word_remp);

    println!("Esta es la cadena original: {:?}", ej_string);
    println!(
        "Esta es la palabra que voy a remplazar: {:?}",
        word_original
    );
    println!("Por esta palabra la remplazo: {:?}", word_remp);
    println!("aca esta el string modificado: {:?}", replacement_string);

    println!("\n---- EJERCICIO 4");
    let string_original = "Hola, Mundo";
    let start_index = 2;
    let end_index = 6;
    match cut_str(string_original, start_index, end_index) {
        Some(sub_str) => println!("subcadena: {}", sub_str),
        None => println!("Los indices estan fuera del rango."),
    }

    println!("\n---- EJERCICIO 5");
    let string_with_char = "Como v! t#$ngo que Co%%e*";

    let remove_chars = delete_char_special(string_with_char);

    println!("Este es la cadena original: {:?}", string_with_char);
    println!(
        "Esta es la cadena con los chars borrados: {:?}",
        remove_chars
    );
}
