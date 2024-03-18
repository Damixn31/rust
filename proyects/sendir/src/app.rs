use crate::{arguments::check_args, config::Config};

pub fn run(config: Config) {
    match check_args() {
        Ok(dir) => {
            println!("Enviano con exito: {}", dir)
        }
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
    println!(
        "Copiando ficheros a {} en {}@{}:{}...",
        config.destiny,
        config.user,
        config.ip,
        config.port //config.port, dir, config.user, config.ip, config.destiny

                    // Lógica para transferir archivos a través de SSH
                    // Por ejemplo, utilizando una biblioteca como ssh2-rs
    );
}
