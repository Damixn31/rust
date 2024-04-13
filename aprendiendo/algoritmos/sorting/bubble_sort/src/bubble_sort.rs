pub fn bubble_sort(arr: &mut [i32]) {
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
