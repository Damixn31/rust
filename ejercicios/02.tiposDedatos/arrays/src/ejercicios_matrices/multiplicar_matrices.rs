pub fn multiplicar(matriz1: &[[i32; 3]; 2], matriz2: &[[i32; 2]; 3]) -> [[i32; 2]; 2] {
    let mut result = [[0; 2]; 2];

    for i in 0..2 {
        for y in 0..2 {
            for x in 0..3 {
                result[i][y] += matriz1[i][x] * matriz2[x][y];
            }
        }
    }
    result
}
