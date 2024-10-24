// crear un enum Transporte que tenga variantes como coche, bicicleta, avion y barco. Escribe una
// funcion que tome un transporte como argumento y devuelva un mensaje diferente segun el tipo de
// transporte.

pub enum Transporte {
    Coche,
    Bicicleta,
    Avion,
    Barco,
}

pub fn type_transport(t: Transporte) {
    match t {
        Transporte::Coche => println!("Esta viajando en coche."),
        Transporte::Bicicleta => println!("Esta viajando en bicicleta."),
        Transporte::Avion => println!("Esta viajando en avion."),
        Transporte::Barco => println!("Esta viajando en barco."),
    }
}
