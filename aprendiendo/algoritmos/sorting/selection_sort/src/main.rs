fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        let mut min_index = i;
        for j in i + 1..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i, min_index)
        }
    }
}
fn main() {
    let mut array = vec![65, 33, 22, 11, 50, 66, 100];
    println!("Array antes de ordenarlo: {:?}", array);
    selection_sort(&mut array);
    println!("Array despues de ordenarlo: {:?}", array);
}
