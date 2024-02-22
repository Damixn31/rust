use rand::seq::SliceRandom;
//pub struct Quotes(Vec<&'static str>);
#[derive(Clone)]
pub struct Quotes {
    pub categories: Vec<Category>,
}

#[derive(Clone)]
pub struct Category {
    pub name: String,
    pub quotes: Vec<&'static str>,
}

impl Quotes {
    // Constructor para inicializar las categorías de citas
    pub fn new() -> Self {
        let categories = vec![
            Category {
                name: "motivacion".to_string(),
                quotes: vec![
                    "La vida es lo que pasa mientras estás ocupado haciendo otros planes. - John Lennon",
                    "Nunca te rindas en algo que realmente quieres. Es difícil esperar, pero peor es arrepentirse.",
                ],
            },
            Category {
                name: "inspiracion".to_string(),
                quotes: vec![
                    "La felicidad es interior, no exterior; por lo tanto, no depende de lo que tenemos, sino de lo que somos. - Henry Van Dyke",
                    "La única forma de hacer un gran trabajo es amar lo que haces. - Steve Jobs",
                ],
            },
            // Puedes agregar más categorías y citas según sea necesario
        ];

        Quotes { categories }
    }

    // Método para obtener una cita aleatoria de una categoría específica
    pub fn random_quote(&self, category_name: &str) -> Option<&'static str> {
        println!("Buscando cita para la categoría: {}", category_name);
        if let Some(category) = self.categories.iter().find(|c| c.name == category_name) {
            if let Some(quote) = category.quotes.choose(&mut rand::thread_rng()) {
                return Some(*quote);
            }
        }
        println!(
            "No se encontró ninguna cita para la categoría: {}",
            category_name
        );
        None
    }
}
