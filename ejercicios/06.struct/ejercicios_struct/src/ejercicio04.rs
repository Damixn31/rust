// 4. Crea una estructura 'Libro' con campos  para el titulo, el autor y el año de la publicacion.
//    Implementa el metodo para imprimir los detalles del titulo.

pub struct Book {
    pub title: String,
    pub author: String,
    pub year_public: u32,
}

impl Book {
    pub fn print_book_details(&self) {
        println!("Titulo: {}", self.title);
        println!("Autor: {}", self.author);
        println!("Año Publicado: {}", self.year_public);
    }
}
