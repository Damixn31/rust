fn main() {
    // Declara una variable mutable llamada 'contador' e inicialízala con el valor 0.
    let mut contador = 0;
    // Incrementa el valor de 'contador' en 1.
    contador += 1;
    // Imprime el nuevo valor de 'contador'.
    println!("El valor nuevo de contador es: {}", contador);

    //Ejercicio 2: Acumular el valor de una variable
    //Crea una variable mutable que acumule un valor en un ciclo. Por ejemplo, suma números del 1 al 5 y muestra el valor acumulado al final.
    println!("---Ejercicio 2----");
    let mut acumul = 0;
    for i in 1..=5 {
	acumul += i;
    };
    println!("El valor acumulado es: {}", acumul);
    

    //    Ejercicio 3: Cambiar el contenido de una tupla
    //Define una tupla mutable y cambia uno de sus elementos.
    let mut tup = (5, "nico", false);

    tup.0 = 10;
    println!("Tupla antes: {:?}", tup);

    

    //Ejercicio 4: Trabajar con String mutable
    //	Crea una cadena mutable y concatena texto adicional a ella varias veces.
    let mut text = String::from("mundo");

    text.push_str(", hola");
    text.push_str(" nico");

    println!("Agregando: {}", text);

    //	Ejercicio 5: Controlar el valor de una variable mutable con condicionales
    //Crea una variable mutable y ajusta su valor según el resultado de una condición.
    let mut valor = 10;

    if valor > 5 {
	valor = valor - 3;
    } else if valor <= 5 {
	valor = valor + 3;
	println!("El valor se diminuyo menos 3: {}", valor);
    } else {
	println!("No hay modificaciones");
    }

    println!("El valor es: {}", valor);
}
