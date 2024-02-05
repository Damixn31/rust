pub fn rotate_matriz(matriz: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut matriz_rotate = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            matriz_rotate[i][j] = matriz[2 - j][i];
        }
    }
    matriz_rotate
}
