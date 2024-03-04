use actix_web::{HttpRequest, HttpResponse, Result};
use std::fs::File;
use std::io::Read;

pub async fn handle_media_request(req: HttpRequest) -> Result<HttpResponse> {
    // obtiene el nombre del archivo solicitado desde la URL
    let filename = req.match_info().query("filename");

    //Ruta donde se almacena los archivos multimedia
    let media_dir = "./media/";

    // Intenta abrir el fichero solicitado
    let file_path = format!("{}{}", media_dir, filename);
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    // Lee el contenido del fichero en un vector de bytes
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Devuelve el contenido del fichero como respuesta Http
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(buffer))
}

