use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Vehicle {
    #[serde(rename(deserialize = "Manufacturer"))]
    manufacturer: String,

    #[serde(rename(deserialize = "Model"))]
    model: String,

    #[serde(rename(deserialize = "VIN"))]
    vin: String,
}

fn main() {
    let file_name = "data.csv";
    let mut builder = ReaderBuilder::new();
    builder
        .double_quote(false)
        .comment(Some(b'*'))
        .delimiter(b'|');
    // si en nuestro data.csv tendria pipe | en ves de , pondiramos el delimiter(b'|')
    let result = builder.from_path(file_name);

    if result.is_err() {
        println!("Failed to read CSV file path");
        std::process::exit(9); // para que salga un error distinto a 0
    }

    let mut my_reader = result.unwrap();
    for record in my_reader.deserialize() {
        let car: Vehicle = record.unwrap();
        println!("tu coche manufacture es {:?}", car.manufacturer);
        println!("tu coche model es {:?}", car.model);
        println!("tu coche VIM es {:?}", car.vin);
    }
}
