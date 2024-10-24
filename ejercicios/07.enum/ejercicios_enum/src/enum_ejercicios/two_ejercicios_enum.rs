// define un enum 'Resultado' con dos variantes: 'Exito' y 'Fallo'. Exito tiene que obtener un
// valor i32 , y fallo debe tener un cadena de error. Escribe una funcion  que reciba un Resultado
// y devuelva un mensaje basado en si fue un exito o u fallo.

pub enum Resultado {
    Exito(i32),
    Fallo(String),
}

pub fn this_is(r: Resultado) -> String {
    match r {
        Resultado::Exito(value) => format!("Exito el valor es: {}", value),
        Resultado::Fallo(err) => format!("Fallo: {}", err),
    }
}
