use crate::random_part::random_partition;

pub fn random_quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivote_index = random_partition(arr);
    random_quick_sort(&mut arr[0..pivote_index]);
    random_quick_sort(&mut arr[pivote_index + 1..]);
}
