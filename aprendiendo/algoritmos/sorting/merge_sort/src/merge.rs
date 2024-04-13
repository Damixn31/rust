//use std::slice;

pub fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    // en la libreria estandar tenemos el slice que hace lo mismo que esta funcion
    // slice::merge(left, right, result);

    let mut left_index = 0;
    let mut right_index = 0;
    let mut result_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            result[result_index] = left[left_index];
            left_index += 1;
        } else {
            result[result_index] = right[right_index];
            right_index += 1;
        }
        result_index += 1;
    }

    while left_index < left.len() {
        result[result_index] = left[left_index];
        left_index += 1;
        result_index += 1;
    }

    while right_index < right.len() {
        result[result_index] = right[right_index];
        right_index += 1;
        result_index += 1;
    }
}
