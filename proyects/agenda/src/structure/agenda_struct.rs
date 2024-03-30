use super::contact_struct::Contact;

#[derive(Debug, Clone)]
pub struct Agenda {
    pub contacts: Vec<Contact>,
}
