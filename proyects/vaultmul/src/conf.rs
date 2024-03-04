use dotenv::dotenv;
use std::env;
pub struct Conf {
    pub port: u16,
    pub host: String,
}

pub fn read_config() -> Conf {
    dotenv().ok();

    Conf {
        port: env::var("PORT")
            .expect("Port no esta definido.")
            .parse()
            .expect("Definicion del port no fue parseada."),
        host: env::var("HOST").expect("Host no esta definido."),
    }
}
