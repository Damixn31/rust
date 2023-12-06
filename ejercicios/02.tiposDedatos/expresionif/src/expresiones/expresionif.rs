pub fn verification_number(number: i32) {
    if number > 0 {
        println!("El numero es positivo");
    } else if number < 0 {
        println!("El numero no negativo");
    } else {
        println!("El numero es cero");
    }
}
