use std::collections::HashSet;

fn main() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array); //imprimer el array

    //1. Ejercicio Crea un array de tamaño 5 que contenga los números del 1 al 5. Accede e imprime el tercer elemento.

    let acceder_tercer_elemento = array[2];
    println!("{:?}", acceder_tercer_elemento);

    //2. Crea un Vec que inicialmente esté vacío. Inserta los números 1, 2 y 3. Luego, elimina el segundo elemento.
    let mut vec: Vec<i32> = Vec::new(); // inicializacion de vector vacio

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!(" vector despues de agregar: {:?}", vec);
    vec.remove(1);
    println!(" vector despues de eliminar: {:?}", vec);

    //3. Toma un Vec con valores [10, 20, 30, 40] y actualiza el tercer valor para que sea 100.
    let mut vec2: [i32; 4] = [10, 20, 30, 40];
    println!("vector antes de actualizar: {:?}", vec2);
    vec2[2] = 100;

    println!("vector despues de actualizar: {:?}", vec2);

    //4.  Dado un Vec con valores [1, 2, 3, 4, 5], suma todos sus elementos y muestra el resultado.

    let vec3: [i32; 5] = [1, 2, 3, 4, 5];
    let result_suma: i32 = vec3.iter().sum();
    println!("El resultado de la suma: {:?}", result_suma);

    //6. Objetivo: Dado un Vec con valores [1, 2, 2, 3, 4, 4, 5], elimina los duplicados y crea un nuevo vector con solo valores únicos.

    let vec4: [i32; 7] = [1, 2, 2, 3, 4, 4, 5];
    let set: HashSet<_> = vec4.into_iter().collect();
    let uniq_vec: Vec<_> = set.into_iter().collect();
    println!("vec con valores unicos: {:?}", uniq_vec);
}
