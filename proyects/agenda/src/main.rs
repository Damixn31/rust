mod implementation;
mod structure;

use structure::agenda_struct::Agenda;

fn main() {
    let mut agenda = Agenda::new_agenda();

    // Aca estamos ingresando datos a la agenda
    agenda.add_contact(
        "Nicolas".to_string(),
        "342342344234234".to_string(),
        "nicopunki@hotmail.com".to_string(),
    );

    agenda.add_contact(
        "Sandra".to_string(),
        "1122432132".to_string(),
        "sandrabibi@hotmail.com".to_string(),
    );
    agenda.add_contact(
        "Milena".to_string(),
        "208821133412".to_string(),
        "O.mil@hotmail.com".to_string(),
    );

    for contact in &agenda.contacts {
        println!(
            "Nombre: {}, Telefono: {}, Email: {}",
            contact.name, contact.phone, contact.address
        );
    }

    // Eliminar un contacto

    if let Some(remove_contact) = agenda.delete_contact("Nicolas") {
        println!(
            "Contacto eliminado: nombre {}, Telefono {}, Email {}",
            remove_contact.name, remove_contact.phone, remove_contact.address
        );
    } else {
        println!("No existe el contacto");
    }
}
