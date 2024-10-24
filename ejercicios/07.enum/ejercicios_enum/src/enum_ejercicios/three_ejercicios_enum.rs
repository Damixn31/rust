// crear un enum 'Forma' con variantes ' circulo', 'Cuadrado' y 'Rectangulo', cada variante debe
// tener los valores necesarios para describir la forma (radio para circulo, lado para cuadrado,
// base y altura para triangulo). Escibe una funcion que tome una forma y calcule el area
// correspondiente

pub enum Forma {
    Circulo(f64),
    Cuadrado(f64),
    Rectangulo(f64, f64),
}

pub fn calcule_area(f: &Forma) -> f64 {
    match f {
        Forma::Circulo(radio) => std::f64::consts::PI * radio * radio,
        Forma::Cuadrado(lado) => lado * lado,
        Forma::Rectangulo(base, altura) => 0.5 * base * altura,
    }
}
