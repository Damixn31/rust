// crear un enum llamado "Color" que reciba un los colores "Red", "Green", "Blue" y "Yellow".
// implementa una funcion que reciba un Color y devuelva una cadena con el nombre del color

pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

pub fn color_name(c: Color) -> String {
    match c {
        Color::Red => String::from("Rojo"),
        Color::Green => String::from("Verde"),
        Color::Blue => String::from("Azul"),
        Color::Yellow => String::from("Amarillo"),
    }
}
