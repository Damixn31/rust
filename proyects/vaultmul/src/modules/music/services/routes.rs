use actix_web::{get, post, web, Responder};

use crate::state::State;

use super::super::domain::Playlist;
use super::datos::{CratePlaylist, Info};

#[get("/playlist")]
async fn playlist(data: web::Data<State>) -> impl Responder {
    let playlists = data.playlist.lock().expect("No se encontro playlists");
    //let mut playlists: Vec<Playlist> = vec![];

    //let p1: Playlist = Playlist {
    //    name: "Damian 2024".to_string(),
    //    songs: vec![],
    //};

    //playlists.push(p1);
    web::Json(playlists.clone())
}

#[get("/playlist/{id}")]
async fn get_playlist(info: web::Path<Info>, data: web::Data<State>) -> impl Responder {
    let playlists = data.playlist.lock().expect("No se encontro playlists");
    //let playlists: Vec<Playlist> = vec![Playlist {
    //    name: "Ezequiel 2024".to_string(),
    //    songs: vec![],
    //}];
    // NOTA: IMPLEMENTAR LA LOGICA SI NO EXISTE EL ID QUE QUEREMOS OBTENER
    let p1: Playlist = playlists[info.id].clone();

    web::Json(p1)
}

#[post("/playlist")]
async fn create_playlist(dato: web::Json<CratePlaylist>, data: web::Data<State>) -> impl Responder {
    let mut playlists = data.playlist.lock().expect("No se encontro playlists");
    let p1 = Playlist {
        name: dato.name.clone(),
        songs: vec![],
    };
    playlists.push(p1.clone());

    web::Json(p1)
}

// PUT

// PATCH

// DELETE

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(playlist);
    cfg.service(get_playlist);
    cfg.service(create_playlist);
}
