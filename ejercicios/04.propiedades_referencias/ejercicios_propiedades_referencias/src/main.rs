mod module {
    pub mod ciclo_de_vida;
    pub mod patrones_y_match;
    pub mod referencia_simple;
    pub mod referencias_multiples;
}

use module::ciclo_de_vida::Persona;
use module::patrones_y_match::handle_result;
use module::referencia_simple::imprimir_referencia;
use module::referencias_multiples::take_two;

fn main() {
    //let num = 42;
    //imprimir_referencia(&num);

    // otro tipo de dato
    let cadena = String::from("Hola Damian");
    imprimir_referencia(&cadena);

    let mut vec = vec![1, 2, 3];
    take_two(&mut vec);
    println!("Vector modificado: {:?}", vec);

    // propiedad de propiedad de vida
    // Borrow Checker
    // modifica un valor a traves de una referencia inmutabley otra mutable, oberva los errores de
    // Borrow Checker

    //let mut x = 5;
    //let referencia_inmutable = &x;
    //let referencia_mutable = &mut x; // esto genera un error

    //println!("{} {}", referencia_inmutable, referencia_mutable);

    let person = Persona::nuevo(String::from("Nicolas"));
    let name = person.obtener_nombre();
    println!("Nombre: {}", name);

    let result_ok = Ok("Todo salio OKEY!");
    let result_err = Err("Algo salio mal");

    handle_result(result_ok);
    handle_result(result_err);
}
