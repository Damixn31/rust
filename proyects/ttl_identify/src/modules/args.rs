use std::{
    env::{self},
    process::exit,
};

pub fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("\n\tUso: {} <IP>", args[0]);
        exit(1);
    }
    args[1].clone()
}
