use std::env;

use crate::email::{create_email::create_email, send_email::send_email};

pub fn print_use() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            "Uso: {} <comando> <destinatario> [mensaje] [ruta_pdf]",
            args[0]
        );
        println!("Comandos disponibles");
        println!(" enviar <destinatario> [mensaje] [ruta_pdf]: Envia el correo electronico al destinatario el pdf es opcional.");
        return;
    }

    match args[1].as_str() {
        "enviar" => {
            if args.len() != 4 {
                println!("Uso: {} enviar <destinatario>", args[0]);
                return;
            }
            let destiny = &args[2];
            let message = &args[3];
            let path_pdf = if args.len() > 4 {
                Some(args[4].as_str())
            } else {
                None
            };
            let email = create_email(destiny, message, path_pdf);
            send_email(&email);
        }
        _ => {
            println!("Comando no reconocido");
        }
    }
}
