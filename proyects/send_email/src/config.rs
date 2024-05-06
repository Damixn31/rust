use dotenv::dotenv;
use std::env;

pub struct Conf {
    pub credential: String,
    pub email: String,
    pub cv: String,
}

pub fn read_config() -> Conf {
    dotenv().ok();

    Conf {
        credential: env::var("CREDENTIAL")
            .expect("credential no defined!")
            .parse()
            .expect("No parser credential."),
        email: env::var("EMAIL")
            .expect("email no defined")
            .parse()
            .expect("No parse Email."),
        cv: env::var("CV")
            .expect("cv no defined!")
            .parse()
            .expect("No parse cv"),
    }
}
