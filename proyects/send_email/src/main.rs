//use email::send_email::send_email;
//use lettre::message::{header::ContentType, Attachment, MultiPart, SinglePart};
//use lettre::transport::smtp::authentication::Credentials;
//use lettre::{Message, SmtpTransport, Transport};
//use send_email::config::read_config;

use menu::menu_use::print_use;

pub mod config;
pub mod email;
pub mod menu;

//use crate::email::create_email::create_email;

fn main() {
    //let email = create_email();
    //send_email(&email);
    print_use();
}
