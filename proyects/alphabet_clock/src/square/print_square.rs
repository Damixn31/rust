//use chrono::prelude::*;
use colored::*;

//let hour_current = chrono::Local::now().format("%H:%M:%S").to_string();

pub fn print_square(position_highlighted: &[(usize, usize)], hour_current: &str) {
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

    let square: Vec<Vec<char>> = words.iter().map(|s| s.chars().collect()).collect();
    println!("la hora es {}", hour_current);

    if position_highlighted.is_empty() {
        for fila_chars in &square {
            for &character in fila_chars {
                print!("{}", format!("{} ", character).bright_black());
            }
            println!();
        }
    } else {
        for (i, fila_chars) in square.iter().enumerate() {
            for (j, &character) in fila_chars.iter().enumerate() {
                if position_highlighted.contains(&(i, j)) {
                    print!("{}", format!("{} ", character).bright_purple());
                } else {
                    print!("{}", format!("{} ", character).bright_black());
                }
            }
            println!()
        }
    }
}
