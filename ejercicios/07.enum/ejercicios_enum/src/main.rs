use ejercicios_enum::enum_ejercicios::{
    eight_ejercicios_enum::{performance_operation, Operation},
    five_ejercicios_enum::{take_event, Event},
    four_ejercicios_enum::{list_menu, Menu},
    one_ejercicios_enum::{type_transport, Transporte},
    seven_ejercicios_enum::{color_name, Color},
    six_ejercicios_enum::{status_evaluacion, Evalucion},
    three_ejercicios_enum::{calcule_area, Forma},
    two_ejercicios_enum::{this_is, Resultado},
};

fn main() {
    // ejercicio 01
    let coche = Transporte::Coche;
    let bicicleta = Transporte::Bicicleta;
    let avion = Transporte::Avion;
    let barco = Transporte::Barco;

    type_transport(coche);
    type_transport(bicicleta);
    type_transport(avion);
    type_transport(barco);

    // ejercicio 02
    let r_exito = Resultado::Exito(42);
    let r_fallo = Resultado::Fallo(String::from("Ocurrio un error"));

    println!("{}", this_is(r_exito));
    println!("{}", this_is(r_fallo));

    // ejercicio 03
    let circulo = Forma::Circulo(5.0);
    let cuadrado = Forma::Cuadrado(4.0);
    let rectangulo = Forma::Rectangulo(3.0, 6.0);

    println!("Area de un circulo es: {}", calcule_area(&circulo));
    println!("Area de un cuadrado es: {}", calcule_area(&cuadrado));
    println!("Area de un rectangulo es: {}", calcule_area(&rectangulo));

    // ejercicio 04
    let pedido = vec![
        Menu::Comida {
            name: String::from("Hamburguesa"),
            prince: 0.5,
        },
        Menu::Bebida {
            name: String::from("Refresco"),
            prince: 2.0,
        },
        Menu::Postre {
            name: String::from("Pastel"),
            prince: 4.5,
        },
    ];
    let total = list_menu(&pedido);
    println!("El costo total de pedido es: {}", total);

    // 05 ejercicio

    let evento_tecla = Event::Keyboard('a');
    let evento_raton = Event::Mouse(100, 200);

    println!("{}", take_event(evento_tecla));
    println!("{}", take_event(evento_raton));

    let evaluacion_aprovado = Evalucion::Aprobado(85);
    let evalucion_desaprovado =
        Evalucion::Desaprobado(String::from("Necesitas mejorar en Ingles."));
    let evalucion_en_proceso = Evalucion::EnProceso;

    println!("{}", status_evaluacion(evaluacion_aprovado));
    println!("{}", status_evaluacion(evalucion_desaprovado));
    println!("{}", status_evaluacion(evalucion_en_proceso));

    // 07 ejercicio
    let my_color = Color::Green;
    let name = color_name(my_color);
    println!("El color es: {}", name);

    // 08 ejercicio
    let a = 10;
    let b = 5;

    let add_result = performance_operation(a, b, Operation::Add);
    let sub_result = performance_operation(a, b, Operation::Subtract);
    let mul_result = performance_operation(a, b, Operation::Multiply);
    let div_result = performance_operation(a, b, Operation::Divide);
    let div_zero = performance_operation(a, 0, Operation::Divide);

    println!("Addittion: {:?}", add_result.unwrap());
    println!("Subtraction: {:?}", sub_result.unwrap());
    println!("Multiplication: {:?}", mul_result.unwrap());
    println!("Division: {:?}", div_result.unwrap());
    match div_zero {
        Some(result) => println!("Division by zero result: {}", result),
        None => println!("Division by zero is undefined!"),
    }
}
