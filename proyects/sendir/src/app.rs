use std::error::Error;

use crate::{arguments::check_args, config::Config, connection::conn_tcp};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //let user = "ezerve";
    //let host = "192.168.0.100";
    //let port = 2323;

    match check_args() {
        Ok(dir) => {
            println!("Enviano con exito: {}", dir)
        }
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
    //match conn_tcp(user, host, port) {
    //    Ok(session) => {
    //        println!("!conexion ssh conectada con exito!");
    //    }
    //    Err(err) => {
    //        eprintln!("Error al establecer la conexion ssh {}", err);
    //        std::process::exit(1);
    //    }
    //}

    println!(
        "Copiando ficheros a {} en {}@{}:{}...",
        config.destiny,
        config.user,
        config.ip,
        config.port //config.port, dir, config.user, config.ip, config.destiny

                    // Lógica para transferir archivos a través de SSH
                    // Por ejemplo, utilizando una biblioteca como ssh2-rs
    );
    Ok(())
}
