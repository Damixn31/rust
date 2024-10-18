fn main() {
    let tup1 = (10, 20);
    let result_inter = intercam(tup1);
    println!("El resultado del intercambio es: {:?}", result_inter);

    //    Ejercicio 2: Suma de elementos
    //Escribe una función que reciba una tupla de tres enteros (i32, i32, i32) y devuelva la suma de los tres elementos.
    let tup2 = (5, 10, 25);
    let result_suma = sum_tup(tup2);
    println!("El resultado de la suma de los tres numero es: {:?}", result_suma);

    //   Ejercicio 3: Información de persona
    //Define una función que acepte una tupla que contenga información sobre una persona: nombre (String), edad (u32) y altura (f32). La función debe imprimir esta información de manera estructurada.
    let persona1 = (String::from("nicolas"), 30, 1.65);
    info_person(persona1);

    //    Ejercicio 4: Descomposición de tuplas
    //Dado una tupla con tres valores enteros, escribe una función que descomponga la tupla en tres variables distintas e imprima cada una por separado.
    let numbers = (42, 76,21);
    descomponer_tup(numbers);

    //  Ejercicio 5: Comparación de tuplas
    //Escribe una función que reciba dos tuplas con dos valores cada una (i32, i32) y compare si las tuplas son iguales o diferentes.
    let t1 = (10, 20);
    let t2 = (10, 20);
    let t3 = (30, 40);

    println!("{}", comparate(t1, t2));
    println!("{}", comparate(t1, t3));

    //    Ejercicio 6: Conversión de tupla a string
    //Escribe una función que tome una tupla de tres valores (i32, i32, i32) y la convierta en una cadena de texto en el formato "(x, y, z)".
    let tup_to_convert = (10, 20, 50);
    let result = convert_tup_str(tup_to_convert);
    println!("{}", result);

}

fn convert_tup_str(valor: (i32, i32, i32)) -> String {
    format!("({}, {}, {})", valor.0, valor.1, valor.2)
}

fn comparate(tup1: (i32, i32), tup2: (i32, i32)) -> bool {
    if tup1 == tup2 {
	true
    } else {
	false
    }
}
fn descomponer_tup(valor: (i32, i32, i32)) {
    let (n1, n2, n3) = valor;
    println!("Primer numero: {}", n1);
    println!("segundo numero: {}", n2);
    println!("tercer numero: {}", n3);
}

fn info_person(valor: (String, u32, f64)) {
    let (nombre, edad, altura) = valor; 
    println!("el nombre es: {}", nombre);
    println!("la edad es: {}", edad);
    println!("la altura es: {:.2} metros", altura);
}

fn intercam(valor: (i32, i32)) -> (i32, i32) {
    let (a, b) = valor;
    (b, a)
}

fn sum_tup(valor: (i32, i32, i32)) -> i32 {
    let (a, b, c) = valor;
    a + b + c
}


