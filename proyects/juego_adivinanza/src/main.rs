use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el numero!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Por favor ingresa tu suposicion de numero para ganar!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Numero demasisiado chico!"),
            Ordering::Greater => println!("Numero muy grande!"),
            Ordering::Equal => {
                println!("Ganaste Felicitaciones!");
                break;
            }
        }
    }
}
