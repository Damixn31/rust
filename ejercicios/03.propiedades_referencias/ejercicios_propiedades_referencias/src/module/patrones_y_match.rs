// crea una funcion que tome un 'Result' y use un bloque 'match' para imprimir  "Exito" si es
// 'Ok(valor)' y Error si es 'Err(error)'
pub fn handle_result(result: Result<&str, &str>) {
    match result {
        Ok(value) => println!("Exito: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}
