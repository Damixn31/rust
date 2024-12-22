use colored::*;
pub fn display_result(ip: &str, ttl: u8, system_type: &str) {
    println!(
        "\n\t 🔸 {}: {} | {}: {} | {} => {}\n",
        "IP".white().bold(),
        ip.cyan(),
        "ttl".magenta().bold(),
        ttl,
        "Sistema".bright_yellow(),
        system_type.yellow()
    );
}
