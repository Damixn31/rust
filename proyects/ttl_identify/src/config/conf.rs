use std::env;

pub fn get_ipinfo_token() -> Result<String, String> {
    env::var("TOKEN").map_err(|_| {
        "Error: no se encontro la variable de entorno 'TOKEN'. por favor configurala.".to_string()
    })
}
