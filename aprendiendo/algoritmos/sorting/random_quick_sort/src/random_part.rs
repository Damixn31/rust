use rand::Rng;

pub fn random_partition(arr: &mut [i32]) -> usize {
    let mut rng = rand::thread_rng();
    let pivot_index = rng.gen_range(0..arr.len());
    arr.swap(pivot_index, arr.len() - 1);

    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}
