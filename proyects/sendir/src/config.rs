use std::env;

#[derive(Debug)]
pub struct Config {
    pub user: String,
    pub port: String,
    pub destiny: String,
    pub ip: String,
}

impl Config {
    pub fn new_conf() -> Self {
        dotenv::dotenv().ok();

        let user = env::var("USER").unwrap();
        let port = env::var("PORT").unwrap();
        let destiny = env::var("DESTINY").unwrap();
        let ip = env::var("IP").unwrap();

        Config {
            user,
            port,
            destiny,
            ip,
        }
    }
}
