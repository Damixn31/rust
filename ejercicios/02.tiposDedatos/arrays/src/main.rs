use std::io;

use crate::ejercicios_matrices::{
    matriz_espiral::espiral, matriz_iguales::matriz_iguales, multiplicar_matrices::multiplicar,
    rotacion_matriz::rotate_matriz, submatriz_maxima::submatriz_maxima, suma_diagonales::sum_diag,
};

mod ejercicios_matrices {
    pub mod matriz_espiral;
    pub mod matriz_iguales;
    pub mod multiplicar_matrices;
    pub mod rotacion_matriz;
    pub mod submatriz_maxima;
    pub mod suma_diagonales;
}

fn main() {
    // multipicar matrices
    let matriz_1: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    let matriz_2: [[i32; 2]; 3] = [[7, 8], [9, 10], [11, 12]];

    let matriz_result = multiplicar(&matriz_1, &matriz_2);

    // comparar si la matrices son iguales

    let matriz3: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    let matriz4: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    let matriz5: [[i32; 3]; 2] = [[10, 33, 31], [14, 35, 56]];

    if matriz_iguales(&matriz3, &matriz4) {
        println!("Matriz 3 es igual a Matriz 4");
    } else {
        println!("No son iguales");
    }

    if matriz_iguales(&matriz3, &matriz5) {
        println!("Matriz 3 es igual a Matriz 5");
    } else {
        println!("No son iguales");
    }

    // rotar matriz 90 grados

    let matriz_cuadrada: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let matriz_rotada = rotate_matriz(&matriz_cuadrada);

    println!("Matriz rotada 90 grados: {:?}", matriz_rotada);

    // suma de suma diagonales

    let matriz_cuadrada: [[i32; 3]; 3] = [[1, 6, 3], [4, 8, 6], [7, 8, 9]];
    let (suma_pri, suma_secun) = sum_diag(&matriz_cuadrada);

    println!("suma diagonal primaria: {:?}", suma_pri);
    println!("suma diagonal secundaria: {:?}", suma_secun);

    println!(
        "Resultado de la matriz de multiplicacion: {:?}",
        matriz_result
    );
    let a = [1, 2, 3, 4, 5];

    // matriz matriz estpiral
    let n: usize = 3;
    let matriz_esp = espiral(n);
    println!("Matriz espira de {:?}x{:?}", matriz_esp, matriz_esp);

    // imprime el array completo
    //println!("El array contiene {:?}", a);

    // imprime cantidad de elementos
    //println!("El tiene {} elementos", a.len());

    // imprime cada elemento en una linea
    //for element in a.iter() {
    //    println!("{}", element);
    //}

    // usando tipado
    //let a: [i32; 5] = [1, 2, 3, 4, 5];

    //println!("El tiene {} elementos", a.len());

    //acceder a elementos de un array
    //let first = a[0];
    //let second = a[1];

    //println!(
    //    "el primer valor es: {}, el segundo valor es: {}",
    //    first, second
    //);

    // intentar acceder a un index mas alla del establecido en un array sale error si le damos a un
    // index que que es mas de 5 en ete caso la variable que asignamos es de 5 elementos
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // submatriz_maxima

    let matriz: Vec<Vec<i32>> = vec![
        vec![8, 8, -1, -4, -20],
        vec![-8, -3, 4, 2, 1],
        vec![3, 8, 10, 1, 3],
        vec![-4, -1, 1, 7, -6],
    ];

    println!("Matriz original:");
    imprimir_matriz(&matriz);

    let (inicio_fila, inicio_columna, fin_fila, fin_columna) = submatriz_maxima(&matriz);

    println!(
        "Submatriz maxima encontrada: [{}, {}] a [{}, {}]",
        inicio_fila, inicio_columna, fin_fila, fin_columna
    );
}

fn imprimir_matriz(matriz: &Vec<Vec<i32>>) {
    for fila in matriz {
        for &elemento in fila {
            print!("{:4}", elemento);
        }
        println!();
    }
}
