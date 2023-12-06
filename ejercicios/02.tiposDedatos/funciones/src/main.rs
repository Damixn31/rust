use std::io;

mod ejercicio_funciones {
    pub mod suma_dos_numeros;
}

use ejercicio_funciones::suma_dos_numeros::suma;

fn main() {
    //lee el primer numero desde la consola
    let mut input = String::new();
    println!("Ingrese el primer numero:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let num1: i32 = input
        .trim()
        .parse()
        .expect("Por favor ingrese un numero valido");

    input.clear();

    //lee el segundo numero desde la consola
    println!("Ingrese el segundo numero:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let num2: i32 = input
        .trim()
        .parse()
        .expect("Por facot ingrese un numero valido");

    let result = suma(num1, num2);
    println!("El resultado de la suma es: {}", result);
}
