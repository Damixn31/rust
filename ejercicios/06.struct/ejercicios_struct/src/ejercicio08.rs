// 8. Crear una estructura Triangulo con campos para la longitud de cada uno de sus lados.
// Implementar métodos para calcular el perímetro y el área del triángulo.
pub struct Triangle {
    pub side_a: f64,
    pub side_b: f64,
    pub side_c: f64,
}

impl Triangle {
    pub fn new_triangle(side_a: f64, side_b: f64, side_c: f64) -> Option<Triangle> {
        if side_a + side_b > side_c && side_a + side_c > side_b && side_b + side_c > side_a {
            Some(Triangle {
                side_a,
                side_b,
                side_c,
            })
        } else {
            None
        }
    }

    pub fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }

    pub fn area_triangle(&self) -> f64 {
        let s = self.perimeter() / 2.0; // se utiliza en el cálculo del semiperímetro (s) porque la fórmula del semiperímetro en un triángulo
                                        // es la mitad del perímetro. Como el perímetro es la suma de las longitudes de los tres lados del triángulo,
                                        // al dividirlo por 2.0, obtenemos la mitad del perímetro, que es el semiperímetro.

        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt() // sqrt() funcion
                                                                               // que calcula la raiz cuadrada de un numero
    }
}
