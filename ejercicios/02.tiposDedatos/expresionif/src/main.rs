use std::io;

mod expresiones {
    pub mod expresionif;
}

use expresiones::expresionif::verification_number;

fn main() {
    let mut input = String::new();
    println!("Ingrese un numero:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let number: i32 = input.trim().parse().expect("Error al convertir a numero");

    verification_number(number)
}
