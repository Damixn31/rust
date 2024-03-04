use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
//use handlers::handle_media_request;

mod app;
mod handlers;

use vaultmul::conf::read_config;
use vaultmul::modules::music;
use vaultmul::modules::music::domain::Playlist;
use vaultmul::services::routes::config;
use vaultmul::state::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conf = read_config();
    let stack: Vec<Playlist> = vec![];
    let playlists = Arc::new(Mutex::new(stack));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                playlist: playlists.clone(),
            }))
            .service(
                web::scope("/api")
                    .configure(config)
                    .configure(music::services::routes::config),
            )
    })
    .bind((conf.host, conf.port))?
    .run()
    .await
}
