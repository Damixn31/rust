//2. Filtrar Números Pares
//Crea una función que reciba un Vec<i32> y devuelva un nuevo vector que solo contenga los números pares.

pub fn only_par(vec: Vec<i32>) -> Vec<i32> {
    let mut par = Vec::new();

    for x in vec {
        if x % 2 == 0 {
            par.push(x);
        }
    }
    par
    //vec.into_iter().filter(|&x| x % 2 == 0).collect()
}
