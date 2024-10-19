//4. Ordenar un Vec de Tuplas
//Dado un vector de tuplas Vec<(String, i32)>, donde el primer elemento es el nombre de una persona y el segundo es su edad,
//escribe una función que ordene el vector en orden ascendente según la edad.
pub fn ord_vec_for_age(mut person: Vec<(String, i32)>) -> Vec<(String, i32)> {
    //person.sort_by(|a, b| a.1.cmp(&b.1));

    // bubble sort
    let n = person.len();

    for i in 0..n {
        for j in 0..n - i - 1 {
            if person[j].1 > person[j + 1].1 {
                person.swap(j, j + 1);
            }
        }
    }
    person
}
