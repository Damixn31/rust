pub fn convert_to_usize_tupla(vec_of_vec: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    vec_of_vec
        .iter()
        .enumerate()
        .flat_map(|(row, row_vec)| row_vec.iter().enumerate().map(move |(col, _)| (row, col)))
        .collect()
}
