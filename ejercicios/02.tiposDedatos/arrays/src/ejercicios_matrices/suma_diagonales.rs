pub fn sum_diag(matriz: &[[i32; 3]; 3]) -> (i32, i32) {
    let mut sum_primary = 0;
    let mut sum_secun = 0;

    for i in 0..3 {
        sum_primary += matriz[i][i];
    }

    for i in 0..3 {
        sum_secun += matriz[i][2 - i];
    }

    (sum_primary, sum_secun)
}
