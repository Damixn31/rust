use std::io;

use ipnetwork::IpNetwork;

fn main() {
    println!("Ingrese direccion IP y CIDR (ejemplo: 192.168.1.10/24):");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la linea");
    let input = input.trim();

    match input.parse::<IpNetwork>() {
        Ok(network) => {
            let network_id = network.network();
            let broadcast = network.broadcast();
            let mask = network.mask();

            println!("Input: {}", input);
            println!("Network ID: {}", network_id);
            println!("Broadcast: {}", broadcast);
            println!("Mask: {}", mask);
        }
        Err(e) => {
            eprintln!("Error al analizar la entrada: {}", e);
        }
    }
}
