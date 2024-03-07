#[derive(Clone, Debug)]
struct Persona {
    name: String,
    ege: u32,
}

fn string_to_uppercase(input: &Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();

    for i in input {
        result.push(i.to_uppercase());
    }
    result
}

fn main() {
    println!("---- Ejercicios de clonacion --------");
    // 1. Clonación de vectores: Crea un programa que tenga un vector de números enteros. Luego, clona ese vector y modifica uno de los vectores. Comprueba si el otro vector también se modifica.
    let mut original = vec![1, 2, 3, 4, 5];

    let clone_original = original.clone();
    let modify = 2;
    original[modify] = 50;

    println!("Este es el el vector original: {:?}", original);
    println!("Este es el el vector clonado: {:?}", clone_original);

    // 2. Copia de estructuras: Define una estructura Persona con campos nombre y edad.
    // Crea una instancia de esta estructura y luego realiza una copia de ella utilizando copy.
    // Modifica la copia y verifica si los cambios se reflejan en la instancia original.
    let mut persona1 = Persona {
        name: String::from("Sandra"),
        ege: 53,
    };

    let persona_copy = persona1.clone();
    let modify_persona = Persona {
        name: String::from("Bruno"),
        ege: 3,
    };
    persona1 = modify_persona;

    println!("primera persona: {:?}", persona1);
    println!("persona copiada: {:?}", persona_copy);

    // 3. Copiando elementos de un vector: Toma un vector de cadenas y crea una función que tome este vector como argumento
    // y devuelva un nuevo vector con todas las cadenas convertidas a mayúsculas.
    // Utiliza copy para realizar la copia de cada cadena.
    let dias = vec!["lunes", "martes", "miercoles"];

    let dias_mayus = string_to_uppercase(&dias);
    println!("Dias en Mayuscula: {:?}", dias_mayus);
}
