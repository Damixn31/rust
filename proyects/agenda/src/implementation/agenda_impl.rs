use crate::structure::{agenda::Agenda, contact::Contact};

impl Agenda {
    pub fn new_agenda() -> Agenda {
        Agenda {
            contacts: Vec::new(),
        }
    }
    pub fn add_contact(&mut self, name: String, phone: String, address: String) {
        let new_contact = Contact {
            name,
            phone,
            address,
        };
        self.contacts.push(new_contact);
    }
}
