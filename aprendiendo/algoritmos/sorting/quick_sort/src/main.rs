use quick_sort::quick::quick_sort;

fn main() {
    let mut arr = vec![4, 2, 6, 19, 3, 5, 20, 11, 9];
    println!("Array Original: {:?}", arr);
    quick_sort(&mut arr);
    println!("Array Ordenado: {:?}", arr)
}
