use crate::structure::{agenda_struct::Agenda, contact_struct::Contact};

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

    pub fn delete_contact(&mut self, name: &str) -> Option<Contact> {
        if let Some(index) = self.contacts.iter().position(|c| c.name == name) {
            Some(self.contacts.remove(index))
        } else {
            None
        }
    }
}
