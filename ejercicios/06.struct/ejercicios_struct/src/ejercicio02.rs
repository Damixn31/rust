// 2. crea una estructura 'Rectangulo' con los campos para ancho y el alto. Implementa metodos para
//    calcular el area y el perimeto del rectangulo

pub struct Rectangule {
    pub width: f64,
    pub height: f64,
}

impl Rectangule {
    pub fn area(&self) -> f64 {
        self.height * self.width
    }
    pub fn perimetro(&self) -> f64 {
        2.0 * (self.height + self.width)
    }
}
