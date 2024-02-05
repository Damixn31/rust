use std::io;

//use chrono::prelude::*;
use colored::*;

use crossterm::{cursor::Hide, ExecutableCommand};

pub fn print_square(position_highlighted: &[(usize, usize)], hour_current: &str) -> io::Result<()> {
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

    if position_highlighted.is_empty() {
        for fila_chars in square.iter().take(10) {
            for &character in fila_chars {
                print!("{}", format!("{} ", character).bright_black());
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

    io::stdout().execute(Hide)?;
    Ok(())
}
