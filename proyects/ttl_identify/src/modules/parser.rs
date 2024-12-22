pub fn parse_ttl(output_str: &str) -> Option<u8> {
    output_str.lines().find_map(|line| {
        if line.contains("ttl=") {
            line.split_whitespace()
                .find(|&part| part.starts_with("ttl="))
                .and_then(|ttl_part| ttl_part[4..].parse::<u8>().ok())
        } else {
            None
        }
    })
}
