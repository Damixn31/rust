// 11. Definir una estructura Punto con campos para las coordenadas x e y.
// Implementar un método para calcular la distancia entre dos puntos.

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new_point(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    pub fn cal_distance(&self, other_point: &Point) -> f64 {
        let dif_x = self.x - other_point.x;
        let dif_y = self.y - other_point.y;
        // fórmula se deriva del teorema de Pitágoras y se representa de la siguiente manera: D = √((x2 - x1)^2 + (y2 - y1)^2)
        (dif_x.powi(2) + dif_y.powi(2)).sqrt()
    }
}
