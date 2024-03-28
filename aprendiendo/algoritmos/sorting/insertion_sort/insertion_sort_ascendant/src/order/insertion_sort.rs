pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let long = arr.len();

    for i in 1..long {
        let mut j = i;

        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
