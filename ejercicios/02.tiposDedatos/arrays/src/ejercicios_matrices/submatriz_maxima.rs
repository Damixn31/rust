pub fn submatriz_maxima(matriz: &Vec<Vec<i32>>) -> (usize, usize, usize, usize) {
    let filas = matriz.len();
    let columnas = matriz[0].len();
    let mut dp = vec![vec![0; columnas]; filas];
    let mut max_suma = i32::MIN;
    let mut max_inicio_fila = 0;
    let mut max_inicio_columna = 0;
    let mut max_fin_fila = 0;
    let mut max_fin_columna = 0;

    for i in 0..filas {
        for j in 0..columnas {
            if i == 0 || j == 0 {
                dp[i][j] = matriz[i][j];
            } else {
                dp[i][j] = matriz[i][j] + dp[i - 1][j] + dp[i][j - 1] - dp[i - 1][j - 1];
            }

            for k in 0..=i {
                for l in 0..=j {
                    let suma = dp[i][j]
                        - if k > 0 { dp[k - 1][j] } else { 0 }
                        - if l > 0 { dp[i][l - 1] } else { 0 }
                        - if k > 0 && l > 0 { dp[k - 1][l - 1] } else { 0 };

                    if suma > max_suma {
                        max_suma = suma;
                        max_inicio_fila = k;
                        max_inicio_columna = l;
                        max_fin_fila = i;
                        max_fin_columna = j;
                    }
                }
            }
        }
    }
    (
        max_inicio_fila,
        max_inicio_columna,
        max_fin_fila,
        max_fin_columna,
    )
}
