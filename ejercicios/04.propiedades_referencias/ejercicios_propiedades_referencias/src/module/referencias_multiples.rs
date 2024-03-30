// Crea una funcion que tome como referencia mutable y modifique el valor referenciado.
// llama a la funcion con un vector y modifica cambios

pub fn take_two(value: &mut Vec<i32>) {
    value.push(200)
}
