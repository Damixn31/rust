pub fn identify_system_by_ttl(ttl: u8) -> &'static str {
    match ttl {
        0..=64 => "Linux / MacOS / BSD",
        65..=128 => "Windows",
        129..=254 => "Route (Cisco)",
        _ => "Desconocido",
    }
}
