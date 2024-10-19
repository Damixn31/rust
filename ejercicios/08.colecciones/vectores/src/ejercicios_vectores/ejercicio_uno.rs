//1. Suma de Elementos en un Vec<i32>
//Escribe una función que reciba un Vec<i32> y devuelva la suma de todos sus elementos.
pub fn sum_element(vec: Vec<i32>) -> i32 {
    vec.iter().sum()
}
