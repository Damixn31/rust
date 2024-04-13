use merge_sort::merge_sort::merge_sort;
fn main() {
    let mut arr = [38, 27, 43, 3, 9, 82, 10];
    println!("--------- Marge sort -------");
    println!("Arr desordenado: {:?}", arr);
    merge_sort(&mut arr);
    println!("Arr Ordenado: {:?}", arr);
}
