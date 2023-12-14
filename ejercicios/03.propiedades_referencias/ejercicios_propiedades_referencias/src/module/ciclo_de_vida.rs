// define una estructura de Persona con un campo nombre  como String
// implementa un metodo nuevo que tome un nombre como String y devuelve una instancia  de Persona
// implementa un metodo obtener_nombre que tome una referencia a 'self' y devuelva una referencias
// de nombre

#[derive(Debug)]
pub struct Persona {
    name: String,
}

impl Persona {
    pub fn nuevo(name: String) -> Persona {
        Persona { name }
    }

    pub fn obtener_nombre(&self) -> &String {
        &self.name
    }
}
