fn es_primos(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..(num / 2 + 1) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

pub fn mostrar_numero_primos(limit: u64) {
    println!("Numeros primos hasta el {}", limit);
    for i in 2..=limit {
        if es_primos(i) {
            println!("{}", i);
        }
    }
}
