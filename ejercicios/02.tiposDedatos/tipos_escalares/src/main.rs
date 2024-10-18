fn main() {
    //1. Suma de enteros y conversión: Escribe un programa en Rust que realice la suma de dos enteros de diferentes tipos (i32 y u32) y conviértelos para que puedan sumarse correctamente.
    let a: i32 = 10;
    let b: u32 = 20;
    // conversion  de a y b  a i32
    let result = a + b as i32;
    println!("El resultado de la suma es: {}", result);

    //2. Operaciones con tipos flotantes: Crea una función que acepte dos números de tipo f64 y realice las siguientes operaciones: suma, resta, multiplicación, división, y módulo (si es aplicable). Muestra los resultados.
    let num1: f64 = 20.0;
    let num2: f64 = 50.0;

    acept_two_numbers(num1, num2);
    //3.Casting entre tipos numéricos: Declara una variable de tipo i8 y otra de tipo u16. Convierte ambas al tipo f32 y realiza una operación matemática entre ellas. Muestra el resultado.
    let v1: i8 = -120;
    let v2: u16 = 65535;

    let convert1: f32 = v1 as f32;
    let convert2: f32 = v2 as f32;

    let suma_convert = convert1 + convert2;

    println!("El resultado de la suma y conversion: {}", suma_convert);
    //4. Booleanos en expresiones: Crea un programa que tenga dos variables booleanas (true y false). Combínalas en expresiones lógicas (AND, OR, NOT) y muestra los resultados de cada operación.
    let a: bool = true;
    let b: bool = false;

    let and_result = a && b;
    println!("a AND b: {}", and_result);

    let or_result = a || b;
    println!("a OR b: {}", or_result);

    let not_a = !a;
    println!("NOT a: {}", not_a);
    //5. Uso de caracteres: Escribe una función que acepte un carácter (char) y devuelva su código Unicode. Luego, convierte el código Unicode en otro carácter y devuélvelo.
    let input_char: char = 'A';
    let (unicode_code, new_char) = unicode_conversion(input_char);

    println!(
        "El caracter '{}' tiene el codigo unicode: {}",
        input_char, unicode_code
    );
    println!("El siguiente caracter es: {}", new_char);

    //6. Operaciones con enteros de diferentes tamaños: Declara variables de los tipos i8, i16, i32, y i64, asigna valores a cada una y realiza una operación matemática que involucre todas ellas. Asegúrate de hacer las conversiones necesarias.
    let k: i8 = -120;
    let j: i16 = 20;
    let u: i32 = -5;
    let l: i64 = -10;

    //let conver_u32: u32 = k as u32;
    let convert_u8: i64 = k as i64;
    let convert_u16: i64 = j as i64;
    let convert_i32: i64 = u as i64;

    let sum_convert = convert_u8 + convert_u16 + convert_i32 + l;

    println!(
        "El resultado de la conversion y la suma de todos es: {}",
        sum_convert
    );
}

fn unicode_conversion(c: char) -> (u32, char) {
    let unicode_code = c as u32;
    let new_char = std::char::from_u32(unicode_code + 1).unwrap_or('�');
    (unicode_code, new_char)
}

fn acept_two_numbers(x: f64, y: f64) {
    let suma = x + y;
    let resta = x - y;
    let multi = x * y;
    let div = x / y;
    let module = x % y;

    println!("Suma: {}", suma);
    println!("resta: {}", resta);
    println!("multiplicación: {}", multi);
    println!("división: {}", div);
    println!("Module: {}", module);
}
