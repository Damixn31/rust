pub fn tabla_de_multiplicar(num: u64) {
    for i in 1..=10 {
        let result = num * i;
        println!("{} x {} = {}", num, i, result);
    }
}
