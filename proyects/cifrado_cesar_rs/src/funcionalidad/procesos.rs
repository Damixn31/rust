use std::io;

use crate::funcionalidad::{cifrado::cifrado_cesar, desifrado::desifrate_cesar};

pub fn process_cypher() {
    let mut msj = String::new();
    let mut key = String::new();

    println!("Ingrese el mensaje que desea cifrar:");
    io::stdin()
        .read_line(&mut msj)
        .expect("Error al leer el mensaje");
    let msj = msj.trim();

    println!("Ingrese la clave (un numero entero):");
    io::stdin()
        .read_line(&mut key)
        .expect("Error al leer la clave");
    let key: i32 = key
        .trim()
        .parse()
        .expect("Por favor, ingrese un numero entero valido");

    let cypher = cifrado_cesar(msj, key);
    let desifrate = desifrate_cesar(&cypher, key);

    println!("Texto cifrado: {:?}", cypher);
    println!("Texto desifrado: {:?}", desifrate);
}
