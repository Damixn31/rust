fn insert_sort_descendent<T: Ord>(arr: &mut [T]) {
    let long = arr.len();

    for i in 1..long {
        let mut j = i;

        while j > 0 && arr[j - 1] < arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn main() {
    let mut numbers = vec![2, 10, 6, 1, 3, 11];

    insert_sort_descendent(&mut numbers);
    println!("Numeros ordenados descendentemente: {:?}", numbers);

    let mut words = vec!["dami", "sandra", "bruno", "agustin", "marco"];
    insert_sort_descendent(&mut words);
    println!(
        "Palabras ordenadas alfabeticamente descendentemente: {:?}",
        words
    );
}
