use vectores::ejercicios_vectores::{
    ejercicio_cuatro::ord_vec_for_age, ejercicio_dos::only_par, ejercicio_tres::word_frequency,
    ejercicio_uno::sum_element,
};

fn main() {
    println!("---Ejercicio 1---");
    let vec1 = vec![1, 2, 3, 4, 5, 6];
    let result_sum = sum_element(vec1);
    println!("El resultado de la suma del vector es: {:?}", result_sum);

    println!("---Ejercicio 2---");

    let vec2 = vec![1, 2, 3, 4, 5, 6, 8, 9, 10];
    let only_pares = only_par(vec2);
    println!(
        "El resultado de todos los numeros pares del vector: {:?}",
        only_pares
    );

    println!("---Ejercicio 3---");
    let text = "hola como estas, hola bien y vos, hola";
    let count_word = word_frequency(text);
    println!("Las palabras que se repiten son: {:?}", count_word);

    println!("---Ejercicio 4---");
    let personas = vec![
        ("juan".to_string(), 25),
        ("ana".to_string(), 30),
        ("pepe".to_string(), 57),
        ("claudia".to_string(), 21),
    ];

    let person_ord = ord_vec_for_age(personas);
    println!("{:?}", person_ord);
}
