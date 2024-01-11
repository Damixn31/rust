use std::io;

//use chrono::prelude::*;
use colored::*;

use crossterm::{cursor::Hide, ExecutableCommand};

//let hour_current = chrono::Local::now().format("%H:%M:%S").to_string();

pub fn print_square(position_highlighted: &[(usize, usize)], hour_current: &str) -> io::Result<()> {
    //let width = 1000;
    //let height = 600;

    //Command::new("kitty")
    //.arg("--start-as=fullscreen")
    //.arg("--initial-window-width")
    //.arg(width.to_string())
    //.arg("--initial-window-height")
    //.arg(height.to_string())
    //.spawn()
    //.expect("No se pudo abrir la terminal Kitty con el tamaño especificado.");

    let words = [
        "ESONELASUNA",
        "DOSITRESOAM",
        "CUATROCINCO",
        "SEISASIETEN",
        "OCHONUEVEPM",
        "LADIEZSONCE",
        "DOCELYMENOS",
        "OVEINTEDIEZ",
        "VEINTICINCO",
        "MEDIACUARTO",
    ];

    let square: Vec<Vec<char>> = words.iter().take(10).map(|s| s.chars().collect()).collect();
    // println!("la hora es {}", hour_current);

    if position_highlighted.is_empty() {
        for fila_chars in square.iter().take(10) {
            for &character in fila_chars {
                print!("{}", format!("{} ", character).bright_black());

                io::stdout().execute(Hide)?;
            }
            println!();
        }
    } else {
        for (i, fila_chars) in square.iter().enumerate().take(10) {
            for (j, &character) in fila_chars.iter().enumerate() {
                if position_highlighted.contains(&(i, j)) {
                    if (i == 1 && j == 9)
                        || (i == 1 && j == 10)
                        || (i == 4 && j == 9)
                        || (i == 4 && j == 10)
                    {
                        print!("{}", format!("{} ", character).bright_cyan());
                    } else {
                        print!("{}", format!("{} ", character).bright_blue());
                    }
                } else {
                    print!("{}", format!("{} ", character).bright_black());
                }
            }
            println!();
        }
    }

    Ok(())
}
