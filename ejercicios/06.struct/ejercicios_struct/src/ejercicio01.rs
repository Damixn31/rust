// 1. Crea una estructura  Persona con campos para el nombre la edad y la altura. Luego implementa
//    una funcion para imprimir los detalles de la persona
pub struct Person {
    pub name: String,
    pub age: u32,
    pub height: f32,
}

impl Person {
    pub fn details_person(&self) {
        println!("Nombre: {}", self.name);
        println!("Edad: {}", self.age);
        println!("Altura: {}", self.height);
    }
}
