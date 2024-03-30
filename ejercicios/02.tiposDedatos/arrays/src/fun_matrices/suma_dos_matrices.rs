pub fn sum_matrices(matriz_one: [[i32; 3]; 2], matriz_two: [[i32; 3]; 2]) -> [[i32; 3]; 2] {
    let mut r = [[0; 3]; 2];

    for i in 0..2 {
        for j in 0..3 {
            r[i][j] = matriz_one[i][j] + matriz_two[i][j]
        }
    }
    r
}
