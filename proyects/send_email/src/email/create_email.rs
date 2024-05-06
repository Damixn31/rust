use lettre::message::header::ContentType;
use lettre::message::{Attachment, MultiPart, SinglePart};
use lettre::Message;

use crate::config::read_config;

pub fn create_email() -> Message {
    let conf = read_config();
    let from_email = format!("Damian Olmedo <{}>", conf.email);
    let message_html = String::from("<h1>Mi nombre es Damian</h1>\n 
        Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.");
    let name_file = String::from("Documento sin titulo-v1.pdf");
    let content_file = std::fs::read(conf.cv).unwrap();
    let attach = Attachment::new(name_file)
        .body(content_file, ContentType::parse("application/pdf").unwrap());

    Message::builder()
        .from(from_email.parse().unwrap())
        .to("<damianol29@hotmail.com>".parse().unwrap())
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
