use bubble_sort::bubble_sort::bubble_sort;

fn main() {
    let mut array = [64, 34, 25, 12, 22, 11, 90];
    println!("Array antes de ordenar: {:?}", array);
    bubble_sort(&mut array);
    println!("Array despues de ordenar: {:?}", array);
}
