pub fn matriz_iguales<T>(matriz_a: &[[T; 3]; 2], matriz_b: &[[T; 3]; 2]) -> bool
where
    T: PartialEq,
{
    for i in 0..2 {
        for j in 0..3 {
            if matriz_a[i][j] != matriz_b[i][j] {
                return false;
            }
        }
    }
    true
}
