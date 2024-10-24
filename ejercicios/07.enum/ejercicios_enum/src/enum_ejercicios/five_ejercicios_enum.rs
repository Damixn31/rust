// crear un enum evento que tenga variables teclado ( que contenga un caracter), y raton (que
// contenga dos i32 para las cordenas x e y). Escribe una funcion que tome un envento y deuvelva un
// mensaje indicando que tipo de evento es y sus detalles.

pub enum Event {
    Keyboard(char),
    Mouse(i32, i32),
}

pub fn take_event(e: Event) -> String {
    match e {
        Event::Keyboard(caracter) => format!("Esto es un evento de caracter: {}", caracter),
        Event::Mouse(x, y) => format!("Esto es un evento de Raton {} {}", x, y),
    }
}
