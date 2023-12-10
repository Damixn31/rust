use std::io;

mod module {
    pub mod factorial;
    pub mod fibonacci;
    pub mod numero_primos;
    pub mod tabla_multiplicar;
}

use module::factorial::factorial_number;
use module::fibonacci::fibonacci;
use module::numero_primos::mostrar_numero_primos;
use module::tabla_multiplicar::tabla_de_multiplicar;

fn main() {
    println!("--------------- contador ascendiente");
    for numero in 1..=10 {
        println!("{}", numero);
    }

    let mut cont = 10;

    println!("--------------- contador descendiente");
    while cont >= 1 {
        println!("{}", cont);
        cont -= 1
    }

    println!("--------------- calcular la suma de numeros pares del 1 al 20");

    let mut sum = 0;
    for number in 1..=20 {
        if number % 2 == 0 {
            sum += number;
        }
    }
    println!("La suma de los numeros pares entre 1 y 20 es: {}", sum);

    println!("--------------- factorial");

    let mut input = String::new();
    println!("Ingrese un numero:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let number: u64 = input.trim().parse().expect("Error al convertir a numero");

    let result = factorial_number(number);

    println!("El numero factorial de {} es: {}", number, result);

    println!("----------------- tabla de multiplicar");

    println!("Ingrese un numero para ver su tabla de multiplicar");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let num: u64 = input.trim().parse().expect("Error al convertir a numero");

    tabla_de_multiplicar(num);

    println!("-------------------- numeros primos");

    println!("Ingrese un limite para mostrar los numeros primos");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let limit: u64 = input.trim().parse().expect("Error al convertir el numero");

    mostrar_numero_primos(limit);

    println!("----------------------- fibonacci");

    println!("Ingresa un numero");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");
    let num: u32 = input.trim().parse().expect("Error al convertir el numero");

    for i in 0..=num {
        println!("{}", fibonacci(i));
    }
}
