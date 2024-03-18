use std::env;

pub fn check_args() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(format!("Error al pasarle los argumentos {}", args[0]));
    }
    Ok(args[1].clone())
}
