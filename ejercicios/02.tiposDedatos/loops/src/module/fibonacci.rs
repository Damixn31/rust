pub fn fibonacci(num: u32) -> u64 {
    if num <= 1 {
        num as u64
    } else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}
