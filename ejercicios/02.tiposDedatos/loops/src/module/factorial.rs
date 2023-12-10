pub fn factorial_number(number: u64) -> u64 {
    let mut result = 1;
    let mut i = 1;

    while i <= number {
        result *= i;
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial_number(0), 1);
        assert_eq!(factorial_number(1), 1);
        assert_eq!(factorial_number(5), 120);
        // Agrega más casos de prueba según sea necesario
    }
}
