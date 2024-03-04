use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Info {
    pub id: usize,
}

#[derive(Debug, Deserialize)]
pub struct CratePlaylist {
    pub name: String,
}
