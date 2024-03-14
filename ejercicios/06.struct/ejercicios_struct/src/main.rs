use ejercicios_struct::ejercicio01::Person;
use ejercicios_struct::ejercicio02::Rectangule;
use ejercicios_struct::ejercicio03::Fecha;
use ejercicios_struct::ejercicio04::Book;
use ejercicios_struct::ejercicio05::Circle;
use ejercicios_struct::ejercicio06::Students;

fn main() {
    // 1. Ejercicio
    let person = Person {
        name: String::from("Damian"),
        age: 34,
        height: 1.67,
    };

    person.details_person();

    // 2. Ejercicio
    let rect = Rectangule {
        width: 5.0,
        height: 3.0,
    };

    println!("Area del rectangulo: {}", rect.area());
    println!("Perimetro del rectangulo:{}", rect.perimetro());

    // 3. Ejercicio
    let fecha = Fecha {
        dia: 6,
        mes: 4,
        año: 1990,
    };

    fecha.imprimir_formato_fecha();

    // 4. Ejercicio
    let libro = Book {
        title: String::from("La casa del terror"),
        author: String::from("Los simpsons"),
        year_public: 1997,
    };

    libro.print_book_details();

    // 5. Ejercicio
    let ciculo1 = Circle::new_circle(5.0);

    println!("El area del circulo es: {}", ciculo1.area());

    let circulo2 = Circle::new_circle(5.0);
    println!(
        "La circunferencia de circulo es: {}",
        circulo2.circunferencia()
    );

    // 6. Ejercicio

    let alumno = Students::new_student(String::from("Bruno"), 23, vec![10, 8, 7, 6, 9]);
    println!(
        "El promedio del alumno {} de ({} Años) es: {}",
        alumno.name,
        alumno.age,
        alumno.average()
    );
}
