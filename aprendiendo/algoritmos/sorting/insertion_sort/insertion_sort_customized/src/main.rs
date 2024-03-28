// Ordenar un Vector de Objetos Personalizados: Crea una estructura personalizada en Rust y luego implementa el trait Ord para esa estructura.
// Despues, intenta ordenar un vector de objetos de esta estructura utilizando el algoritmo de ordenamiento por inserción.

#[derive(Debug, PartialEq, Eq, Clone)]
struct Person {
    name: String,
    age: u32,
}

impl Ord for Person {
    // cmp se usa para permitir la comparación de valores de un tipo para determinar su orden relativo.
    // y devuelve un valor de tipo Ordering -> esto dentro tiene tres posibilidades de orden:
    // Less: indeca que el primer valor es menor que el segundo
    // Equal: inidca que los dos valores son iguales
    // Greater: indica que el primer valor es mayor que el segundo
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        //primero, comparo por edad
        match self.age.cmp(&other.age) {
            // si las edades son direfente retorname el resultado de la comparacion de edad
            std::cmp::Ordering::Equal => self.name.cmp(&other.name), // si las edades son  iguales
            // desempatamos comparando por nombre

            // si las edades son direfentes, retorna el resultado de la comparacion edades
            other => other,
        }
    }
}
// implementacion del trait PartialOrd para la estructura Person
impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn main() {
    let persons = vec![
        Person {
            name: "Sandra".to_string(),
            age: 52,
        },
        Person {
            name: "Nicolas".to_string(),
            age: 30,
        },
        Person {
            name: "Bruno".to_string(),
            age: 3,
        },
        Person {
            name: "Arturo".to_string(),
            age: 54,
        },
    ];

    let mut person_order = persons.clone();
    insert_sort(&mut person_order);

    println!("Personas ordenadas por edad y nombre: {:?}", person_order);
}

fn insert_sort<T: Ord>(arr: &mut [T]) {
    let long = arr.len();

    for i in 1..long {
        let mut j = i;

        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1
        }
    }
}
