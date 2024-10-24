// define un enum llamado "Operation" que tenga variantes para "Add", "Subtract", "Multiply"
// y "Divide". despues implementa una funcion que reciba enteros y un "Operation" y devuelva el
// resultado de la operacion.

pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn performance_operation(a: i32, b: i32, o: Operation) -> Option<i32> {
    match o {
        Operation::Add => Some(a + b),
        Operation::Subtract => Some(a - b),
        Operation::Multiply => Some(a * b),
        Operation::Divide => {
            if b != 0 {
                Some(a / b)
            } else {
                None
            }
        }
    }
}
