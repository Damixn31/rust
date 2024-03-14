// 5. Definir una estructura 'Circulo' con un campo para el radio. Implementar metodos para
//    calcularel area y la circunferencia del circulo
pub struct Circle {
    pub radio: f64,
}

impl Circle {
    pub fn new_circle(radio: f64) -> Circle {
        Circle { radio }
    }

    pub fn area(&self) -> f64 {
        let pi = std::f64::consts::PI;
        pi * self.radio.powi(2) // powi() es un metodo que calcula de un numero entero a un
                                // exponente entero
    }

    pub fn circunferencia(&self) -> f64 {
        let pi = std::f64::consts::PI;
        2.0 * pi * self.radio
    }
}
