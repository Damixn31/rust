use std::fmt::Debug;

#[derive(Debug, Clone)]
struct SortedArray<T> {
    data: Vec<T>,
}

impl<T: Ord + Debug + Clone> SortedArray<T> {
    fn new_arry() -> SortedArray<T> {
        SortedArray { data: Vec::new() }
    }

    // metodo pra insertar elementos y ordenarlos de manera ascendente
    fn insert(&mut self, item: T) {
        let idx = match self.data.binary_search(&item) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        self.data.insert(idx, item);
    }

    // metodo para buscar un elemento
    fn search(&mut self, item: &T) -> Option<&T> {
        self.data
            .binary_search(item)
            .ok()
            .map(|idx| &self.data[idx])
    }

    // metodo para borrar un elmento del array
    fn delete(&mut self, item: &T) -> bool {
        if let Ok(idx) = self.data.binary_search(item) {
            self.data.remove(idx);
            true
        } else {
            false
        }
    }

    // metodo para saber el largo del array
    fn long_array(&self) -> usize {
        self.data.len()
    }

    // metodo para encontrar el primer elemento de un array
    fn first_element(&self) -> Option<&T> {
        self.data.first()
    }

    // metodo para encontrar el ultimo elemento de un array
    fn last_element(&self) -> Option<&T> {
        self.data.last()
    }

    // metodo para obtener elementos unicos
    // lo que tengo que hacer es retornorar un nuevo array sin que alla duplicados dentro del array
    // original
    fn uniq_element(&self) -> SortedArray<T> {
        let mut uniq_array = SortedArray::new_arry();
        let mut prev_item: Option<&T> = None;

        for item in &self.data {
            if prev_item.is_none() || prev_item.unwrap() != item {
                uniq_array.insert(item.clone());
            }
            prev_item = Some(item);
        }
        uniq_array
    }

    //metodo para volver a ordenar un array en el caso de que modifiquemos algo en el array
    fn order_arr(&mut self) {
        self.data.sort()
    }

    // metodo para filtrar segun algun criterio
    fn filter_array<F>(&self, f: F) -> SortedArray<T>
    where
        F: Fn(&T) -> bool,
    {
        let mut result = SortedArray::new_arry();
        for item in &self.data {
            if f(item) {
                result.insert(item.clone());
            }
        }
        result
    }

    // metodo para mapear los elementos del array
    fn mapper_array<U, F>(&self, f: F) -> SortedArray<U>
    where
        F: Fn(&T) -> U,
        U: Ord + Debug + Clone,
    {
        let mut result = SortedArray::new_arry();
        for item in &self.data {
            result.insert(f(item));
        }
        result
    }

    //metodo para imprimir
    fn print(&self) {
        for item in &self.data {
            println!("{:?}", item);
        }
    }
}

fn main() {
    //agregar elementos a un nuevo array y los odena ascendentemente
    let mut sorted_array = SortedArray::new_arry();

    sorted_array.insert(10);
    sorted_array.insert(5);
    sorted_array.insert(2);
    sorted_array.insert(2);
    sorted_array.insert(2);
    sorted_array.insert(8);
    sorted_array.insert(8);
    sorted_array.insert(8);
    sorted_array.insert(8);
    sorted_array.insert(13);
    sorted_array.insert(3);
    sorted_array.insert(3);

    sorted_array.print();

    // buscar el elemento en el array
    let element = 3;
    match sorted_array.search(&element) {
        Some(value) => println!("El elemento {} se encontro en el array", value),
        None => println!("El elemeto {} no se encontro en el array", element),
    }

    // eliminar un elemento del array
    let delete_element = 13;
    if sorted_array.delete(&delete_element) {
        println!("El elemento {} ha sido eliminado", delete_element);
    } else {
        println!(
            "El elemento {} no se pudo eliminar del array",
            delete_element
        );
    }

    sorted_array.print();

    let size = sorted_array.long_array();
    println!("El tamaño del array es: {}", size);

    match sorted_array.first_element() {
        Some(value) => println!("Este es el primero elemento del array {}", value),
        None => println!("El array esta vacio."),
    }

    match sorted_array.last_element() {
        Some(value) => println!("El ultimo elemento del array es: {}", value),
        None => println!("El array esta vacio."),
    }

    let unique_array = sorted_array.uniq_element();

    println!("Elementos sin repetirse:");
    unique_array.print();

    sorted_array.order_arr();

    println!("Array ordenado de manera ascendente:");
    sorted_array.print();

    // filtrar los elementos mayores a 3
    let filtered_array = sorted_array.filter_array(|&x| x > 3);

    println!("Elementos mayores que 3:");
    filtered_array.print();

    // multiplica cada elemento por dos
    let mapper_array = sorted_array.mapper_array(|x| x * 2);

    println!("Array despues de multiplicar cada elemento por 2:");
    mapper_array.print();
}
