use email::send_email::send_email;
//use lettre::message::{header::ContentType, Attachment, MultiPart, SinglePart};
//use lettre::transport::smtp::authentication::Credentials;
//use lettre::{Message, SmtpTransport, Transport};
//use send_email::config::read_config;

pub mod config;
pub mod email;

use crate::email::create_email::create_email;

fn main() {
    let email = create_email();
    send_email(&email);
    //let conf = read_config();

    //let from_email = format!("Damian Olmedo <{}>", conf.email);
    // mensaje con html
    //let message_html = String::from("<h1>Saludos desde rust Damian!</h1>");

    //let name_file = String::from("Documento sin título-v1.pdf");

    //let content_file = std::fs::read("Documento sin título-v1.pdf").unwrap();

    //let attach = Attachment::new(name_file)
    //    .body(content_file, ContentType::parse("application/pdf").unwrap());
    // de finicion de correo
    //let email = Message::builder()
    //    .from(from_email.parse().unwrap())
    //    .to("<damianol29@hotmail.com>".parse().unwrap())
    //    .subject("Saludo!")
    //    .multipart(
    //        MultiPart::mixed()
    //            .singlepart(
    //                SinglePart::builder()
    //                    .header(ContentType::TEXT_HTML)
    //                    .body(message_html),
    //            )
    //            .singlepart(attach),
    //    )
    //    .unwrap();

    // conectar al servicio smtp
    //let credential = Credentials::new("exequiel.ol29".to_owned(), conf.credential.to_owned());

    //let smtp = SmtpTransport::relay("smtp.gmail.com")
    //    .unwrap()
    //    .credentials(credential)
    //    .build();

    //enviar el correo
    //match smtp.send(&email) {
    //    Ok(_) => println!("El correo se envio exitosamente!"),
    //    Err(e) => println!("El error no pudo enviarse {:?}", e),
    // }
}
