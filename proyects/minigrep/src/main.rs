use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // unwrap_or_else -> nos permite definir un manejo de error sin panic!
        println!("Problem parsing arguments: {}", err);
        // process::exit -> es una funcion que detiene el programa inmediatamente y devuelve el
        // numero  que se paso como codigo de estado  de salida
        process::exit(1);
    });
    //let query = &args[1];
    //let filename = &args[2];

    //println!("Buscando por: {}", config.query);

    //println!("En el fichero: {}", config.filename);

    //let contents =
    //    fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    //println!("With text:\n{}", contents);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
    }
}
