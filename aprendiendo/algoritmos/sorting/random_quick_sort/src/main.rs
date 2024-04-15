use random_quick_sort::random_quick::random_quick_sort;

fn main() {
    let mut arr = vec![77, 12, 13, 42, 24, 5, 1, 4, 10, 7];
    println!("Array Original: {:?}", arr);
    random_quick_sort(&mut arr);
    println!("Array Orndenado: {:?}", arr);
}
