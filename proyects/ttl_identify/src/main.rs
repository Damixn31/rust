use std::process::exit;

use dotenv::dotenv;
use ttl_identify::{config::conf::get_ipinfo_token, run::runing::run_program};

#[tokio::main]
async fn main() {
    dotenv().ok();
    match get_ipinfo_token() {
        Ok(token) => {
            if let Err(e) = run_program(token).await {
                eprintln!("{}", e);
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
