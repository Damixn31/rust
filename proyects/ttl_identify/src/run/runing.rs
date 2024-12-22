use crate::modules::{
    args::parse_args, display::display_result, parser::parse_ttl, ping::run_ping,
    recon_ttl::identify_system_by_ttl,
};
use colored::*;

pub fn run_program() -> Result<(), String> {
    let ip = parse_args();
    let output_str = run_ping(&ip)?;
    if let Some(ttl) = parse_ttl(&output_str) {
        let system_type = identify_system_by_ttl(ttl);
        display_result(&ip, ttl, system_type);
        Ok(())
    } else {
        Err("\n\t [❗] No se pudo encontrar el valor del TTL"
            .red()
            .to_string())
    }
}
