use insertion_sort::order::insertion_sort::insertion_sort;

fn main() {
    let mut numbers = vec![4, 2, 5, 3, 1];
    insertion_sort(&mut numbers);
    println!("Numeros ordenados: {:?}", numbers);

    let mut word = vec!["rust", "is", "awesome", "language"];
    insertion_sort(&mut word);
    println!("Palabras ordenadas alfabeticamente: {:?}", word);
}
