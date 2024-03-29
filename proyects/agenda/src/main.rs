mod structure;
use agenda::structure::agenda::Agenda;

fn main() {
    let mut agenda = Agenda::new_agenda();

    agenda.add_contact(
        "Nicolas".to_string(),
        "342342344234234".to_string(),
        "nicopunki@hotmail.com".to_string(),
    );

    for contact in &agenda.contacts {
        println!(
            "Nombre: {}, Telefono: {}, Email: {}",
            contact.name, contact.phone, contact.address
        );
    }
}
