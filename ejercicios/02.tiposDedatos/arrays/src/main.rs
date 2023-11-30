fn main() {
    //let a = [1, 2, 3, 4, 5];

    // imprime el array completo
    //println!("El array contiene {:?}", a);

    // imprime cantidad de elementos
    //println!("El tiene {} elementos", a.len());

    // imprime cada elemento en una linea
    //for element in a.iter() {
    //    println!("{}", element);
    //}

    // usando tipado
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("El tiene {} elementos", a.len());
}
