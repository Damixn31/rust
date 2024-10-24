// definir un enum 'Evalucion' con varieantes 'aprobado', 'desaprobado' y 'en proceso'
//  aprobado contiene una clasificacio  i32
//  desaprobado es un mensaje de retroalimentacion (string)
//  en proceso no contiene datos adicionales
// Escribe una funcion que recibe una evaluacion y devuelva un mensaje indicando el estado de la
// evaluacion

pub enum Evalucion {
    Aprobado(i32),
    Desaprobado(String),
    EnProceso,
}

pub fn status_evaluacion(e: Evalucion) -> String {
    match e {
        Evalucion::Aprobado(clasificacion) => {
            format!("Aprobado con una clasificacion de: {}", clasificacion)
        }
        Evalucion::Desaprobado(mensaje) => format!("Desaprobado retroalimentacion {}", mensaje),
        Evalucion::EnProceso => String::from("La evaluacion esta en proceso"),
    }
}
