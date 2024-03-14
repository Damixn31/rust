use ejercicios_struct::ejercicio01::Person;
use ejercicios_struct::ejercicio02::Rectangule;
use ejercicios_struct::ejercicio03::Fecha;
use ejercicios_struct::ejercicio04::Book;
use ejercicios_struct::ejercicio05::Circle;
use ejercicios_struct::ejercicio06::Students;
use ejercicios_struct::ejercicio07::BankAccount;
use ejercicios_struct::ejercicio08::Triangle;
use ejercicios_struct::ejercicio09::Employee;
use ejercicios_struct::ejercicio10::Vehicle;
use ejercicios_struct::ejercicio11::Point;

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

    // 7. Ejercicio

    let mut cuenta =
        BankAccount::new_account(String::from("Marco"), String::from("54554412331322"), 100.0);

    cuenta.deposit(500.0);
    println!("Deposito exitoso. Saldo actual: {}", cuenta.check_balance());

    match cuenta.extract(700.0) {
        Ok(()) => println!("Retiro exitoso. Saldo actual: {}", cuenta.check_balance()),
        Err(e) => println!("Error al retirar: {}", e),
    }

    // 8. Ejercicio

    let side_a = 5.0;
    let side_b = 3.0;
    let side_c = 4.0;
    if let Some(triangulo) = Triangle::new_triangle(side_a, side_b, side_c) {
        println!("Perimetro del Triangulo: {}", triangulo.perimeter());
        println!("Area del Triangulo: {}", triangulo.area_triangle());
    } else {
        println!("No es posible formar un triangulo con esas longitudes de lado.");
    }

    // 9. Ejercicio
    let empleado = Employee::new_employee(String::from("Martina"), 30.0, 50.0);

    println!(
        "El salario semanal del empleado {} es: ${}",
        empleado.name,
        empleado.weekly_salary()
    );
    // 10. Ejercicio

    let vehiculo = Vehicle {
        brand: String::from("Toyota"),
        model: String::from("Camry"),
        year: 2022,
    };

    vehiculo.details_vehicle();

    // 11. Ejercicio

    let punto1 = Point::new_point(0.5, 0.4);
    let punto2 = Point::new_point(0.0, 0.7);

    let distancia = punto1.cal_distance(&punto2);
    println!("La distancia entre los puntos es: {}", distancia)
}
