use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

use crate::config::read_config;

pub fn send_email(email: &Message) {
    let conf = read_config();
    let credential = Credentials::new("exequiel.ol29".to_owned(), conf.credential.to_owned());
    let smtp = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(credential)
        .build();

    match smtp.send(email) {
        Ok(_) => println!("El correo se envio exitosamente!"),
        Err(e) => println!("Error al enviar en correo {:?}", e),
    }
}
