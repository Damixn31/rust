pub fn espiral(n: usize) -> Vec<Vec<i32>> {
    let mut matriz = vec![vec![0; n]; n];
    let mut valor = 1;
    let (mut primer_fila, mut fin_fila, mut col_inicial, mut col_final) = (0, n - 1, 0, n - 1);

    while valor <= n as i32 * n as i32 {
        for j in col_inicial..=col_final {
            matriz[primer_fila][j] = valor;
            valor += 1;
        }
        primer_fila += 1;

        for i in primer_fila..=fin_fila {
            matriz[i][col_final] = valor;
            valor += 1;
        }
        col_final = col_final.wrapping_sub(1);

        for j in (col_inicial..=col_final).rev() {
            matriz[fin_fila][j] = valor;
            valor += 1;
        }
        fin_fila = fin_fila.wrapping_sub(1);

        for i in (primer_fila..=fin_fila).rev() {
            matriz[i][col_inicial] = valor;
            valor += 1;
        }
        col_inicial += 1;
    }
    matriz
}
