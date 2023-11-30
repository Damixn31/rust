use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

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
}
