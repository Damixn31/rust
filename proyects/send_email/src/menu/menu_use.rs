use std::env;

use crate::email::{create_email::create_email, send_email::send_email};

pub fn print_use() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: {} <comando>", args[0]);
        println!("Comandos disponibles");
        println!(" enviar <destinatario>: Envia el correo electronico al destinatario.");
        return;
    }

    match args[1].as_str() {
        "enviar" => {
            if args.len() != 3 {
                println!("Uso: {} enviar <destinatario>", args[0]);
                return;
            }
            let destinatario = &args[2];
            let email = create_email(destinatario);
            send_email(&email);
        }
        _ => {
            println!("Comando no reconocido");
        }
    }
}
