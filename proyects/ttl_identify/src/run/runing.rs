use crate::{
    api::get_ipinfo::get_ip_info,
    modules::{
        args::parse_args, display::display_result, is_private::is_private_ip, parser::parse_ttl,
        ping::run_ping, recon_ttl::identify_system_by_ttl,
    },
};
use colored::*;

pub async fn run_program(token: String) -> Result<(), String> {
    let ip = parse_args();

    let output_str = run_ping(&ip)?;

    if let Some(ttl) = parse_ttl(&output_str) {
        let system_type = identify_system_by_ttl(ttl);

        if is_private_ip(&ip) {
            display_result(&ip, ttl, system_type);
            let message_ip = "🔒 La IP es privada, no hay información adicional";
            println!("\t{}", message_ip.red());
        } else {
            match get_ip_info(&ip, &token).await {
                Ok(ip_info) => {
                    display_result(&ip, ttl, system_type);
                    println!("\tInformacion:{}", ip_info.display());
                }
                Err(err) => eprintln!(
                    "\n\t [❗] No se pudo obtener información de IPInfo: {}",
                    err
                ),
            }
        }

        Ok(())
    } else {
        Err("\n\t [❗] No se pudo encontrar el valor del TTL"
            .red()
            .to_string())
    }
}
