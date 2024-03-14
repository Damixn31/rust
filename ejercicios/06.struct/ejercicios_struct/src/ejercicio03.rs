// 3. Crea una estructura 'Fecha' con los campos dia, el mes y el año. Implementa el metodo para
//    imprimir la fecha en formato "DD/MM/AAAA"

pub struct Fecha {
    pub dia: u32,
    pub mes: u32,
    pub año: u32,
}

impl Fecha {
    pub fn imprimir_formato_fecha(&self) {
        println!("{}/{}/{}", self.dia, self.mes, self.año)
    }
}
