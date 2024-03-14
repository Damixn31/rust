// 10. Crear una estructura Vehiculo con campos para la marca,
// el modelo y el año de fabricación. Implementar un método para imprimir los detalles del vehículo.

pub struct Vehicle {
    pub brand: String,
    pub model: String,
    pub year: u32,
}

impl Vehicle {
    pub fn details_vehicle(&self) {
        println!("Marca: {:?}", self.brand);
        println!("Modelo: {:?}", self.model);
        println!("Año: {:?}", self.year);
    }
}
