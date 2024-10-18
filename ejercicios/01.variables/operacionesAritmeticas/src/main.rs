fn main() {
    // Realiza las siguientes operaciones:
    // - Suma de 'num1' y 'num2'
    // - Resta de 'num1' y 'num2'
    // - Multiplicación de 'num1' y 'num2'
    // - División de 'num1' entre 'num2'
    // Imprime los resultados de cada operación.   // Declara dos variables, 'num1' y 'num2', e inicialízalas con valores numéricos.
    let num1 = 20;
    let num2 = 5;

    let sum = num1 + num2;
    let resta = num1 - num2;
    let multi = num1 * num2;
    let div = num1 / num2;

    println!("suma: {}, resta: {}, Multiplicación: {}", sum, resta, multi);

    if num2 != 0 {
        println!("la División: {}", div);
    } else {
        println!("No se puede dividir por cero");
    }
}
