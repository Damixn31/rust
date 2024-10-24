// define un enum 'Menu' con variantes comida, Bebida y postres. Cada variante debe tener un nombre
// y un precio. Escribe una funcion que reciba una lista  de Menu y calcule el costo total.

pub enum Menu {
    Comida { name: String, prince: f64 },
    Bebida { name: String, prince: f64 },
    Postre { name: String, prince: f64 },
}

pub fn list_menu(m: &[Menu]) -> f64 {
    let mut total = 0.0;

    for item in m {
        match item {
            Menu::Comida { prince, .. } => total += *prince,
            Menu::Bebida { prince, .. } => total += *prince,
            Menu::Postre { prince, .. } => total += *prince,
        }
    }
    total
}
