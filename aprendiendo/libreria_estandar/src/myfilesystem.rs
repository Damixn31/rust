use std::fs;

// crea un directorio
pub fn create_directory() {
    let path = "./data";
    let my_path = std::path::Path::new(path);

    if my_path.exists() {
        println!("El directorio ya existe...");
        return;
    }

    // tarea
    // verificar permisos -> si tengo permisos en el directorio principal donde queremos crearlo,
    // lo omitiremos
    // o si no temos permisos crearlo nuevamente

    let create_dir_result = fs::create_dir("./data");
    if create_dir_result.is_ok() {
        println!("Nuevo directorio creado!.");
    } else {
        println!(
            "Algo salio mal al crear en nuevo directorio: {:?}",
            create_dir_result.err()
        );
    }
}

// Elininar un directorio
//pub fn remove_dir() {
//let path = "./data";
// si le pasamos a un directorio con fichero adentro con remove_dir() no va a funcionar tenesmos
// que pasarle el remove_dir_all()
//_ = std::fs::remove_dir(path);
//_ = std::fs::remove_dir_all(path);
//}

// crea ficheros
pub fn create_files() {
    let path1 = "./data/file01.txt";
    let path2 = "./data/file02.txt";
    let path3 = "./data/file03.txt";

    let text1 = "Damian Olmedo";
    let text2 = "Marco Olmedo";
    let text3 = "Nicolas Olmedo";

    _ = std::fs::write(path1, text1);
    _ = std::fs::write(path2, text2);
    _ = std::fs::write(path3, text3);

    // remover un fichero
    //_ = std::fs::remove_file(path2);
}

// leer el contenido de un fichero
pub fn read_somefiles() {
    let file_to_read = "./data/file01.txt";
    let read_result = std::fs::read(file_to_read);

    let convert_byte_to_string = |mut a: String, v: &u8| {
        let new_char = char::from(*v);
        a.push(new_char);
        a
    };

    if read_result.is_ok() {
        // fold() ->  se usa en colecciones para reducir todos los elementos de la colección a un solo valor.
        println!(
            "Datos de fichero: {}",
            read_result
                .ok()
                .unwrap()
                .iter()
                .fold(String::from(""), convert_byte_to_string)
        );
    }
}
