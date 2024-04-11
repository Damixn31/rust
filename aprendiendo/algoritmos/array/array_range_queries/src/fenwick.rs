// definir la estructura para representar el arbol de fenwick
#[derive(Debug)]
pub struct FenwinkTree {
    pub tree: Vec<i32>,
}

// implementa el metodo new_fenwink_tree para representar el arbol
impl FenwinkTree {
    pub fn new_fenwick_tree(size: usize) -> Self {
        FenwinkTree {
            tree: vec![0; size + 1],
        }
    }
    // funcion para calcular la suma acumulativa hasta un indice dado en el arreglo
    pub fn query(&self, mut idx: usize) -> i32 {
        let mut sum = 0;
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & idx.wrapping_neg(); // el método wrapping_neg() para obtener el complemento a dos del valor idx. Esta operación es necesaria para obtener el bit menos significativo en la representación binaria del número, que es lo que necesitamos para avanzar al siguiente nodo en el árbol de Fenwick.
        }
        sum
    }

    // funcion para actualizar el valor de un elemento en el arreglo o en el arbol
    pub fn update(&mut self, mut idx: usize, elemet: i32) {
        let n = self.tree.len();
        while idx < n {
            self.tree[idx] += elemet;
            idx += idx & idx.wrapping_neg();
        }
    }
}
