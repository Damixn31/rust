use colored::*;
use std::{collections::HashMap, io::stdout};

use crossterm::{
    cursor::{self, Hide},
    terminal, ExecutableCommand,
};

pub fn clear_and_update_terminal(ip_attempts: &HashMap<String, u32>, connected_ips: &[String]) {
    let mut stdout = stdout();
    stdout.execute(Hide).unwrap(); //volver invisible el cursor
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();

    println!(
        "{}",
        "\n\t\tMonitoreando intentos de ingresos".green().bold()
    );
    println!(
        "{}",
        "\t---------------------------------------------------"
            .white()
            .bold()
    );

    for (ip, count) in ip_attempts {
        println!("\n\t 🔸 IP:  {} - Intentos : {}", ip.red().bold(), count);
    }

    for ip in connected_ips {
        println!("\t✅ Conexion activa desde IP: {}", ip);
    }
}
