// 6. Crea una estructura 'Estudiante' con campos para nombre, edad, un vector de
//    calificaciones(puedes usar un array o un vector). Implementar un metodo para calcular el
//    promedio  de las clasificaciones del estudiante

pub struct Students {
    pub name: String,
    pub age: u32,
    pub rating: Vec<i32>,
}

impl Students {
    pub fn new_student(name: String, age: u32, rating: Vec<i32>) -> Students {
        Students { name, age, rating }
    }

    pub fn average(&self) -> f64 {
        let suma: i32 = self.rating.iter().sum();

        suma as f64 / self.rating.len() as f64 // as, está indicando al compilador que convierta un valor de un tipo a otro.
    }
}
