use std::process::exit;

use ttl_identify::run::runing::run_program;

fn main() {
    if let Err(e) = run_program() {
        eprintln!("{}", e);
        exit(1);
    }
}
