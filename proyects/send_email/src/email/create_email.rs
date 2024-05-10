use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::Message;

use crate::config::read_config;

pub fn create_email(destinatario: &str) -> Message {
    let conf = read_config();
    let from_email = format!("Damian Olmedo <{}>", conf.email);
    let message_html = String::from(
        "<h1>Helowww</h1>\n 
        manu estoy sin whasap estoy en discord por cualquier cosa",
    );
    let name_file = String::from("Documento sin titulo-v1.pdf");
    let content_file = std::fs::read(conf.cv).unwrap();
    let attach = Attachment::new(name_file)
        .body(content_file, ContentType::parse("application/pdf").unwrap());

    Message::builder()
        .from(from_email.parse().unwrap())
        .to(destinatario.parse().unwrap())
        .subject("enviando CV!")
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(message_html),
                )
                .singlepart(attach),
        )
        .unwrap()
}
