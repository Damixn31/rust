use crate::merge::merge;

pub fn merge_sort(arr: &mut [i32]) {
    let leng = arr.len();

    if leng <= 1 {
        return;
    }

    let mid = leng / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    let mut result = vec![0; leng]; // para tenga la misma longitud que la suma de las longitudes de las mitades izquierda y derecha.
    merge(left, right, &mut result[..leng]);
    arr.copy_from_slice(&result);
}
