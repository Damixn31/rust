pub fn fahrenheit_to_celsius(fah: f32) -> f32 {
    (fah - 32.0) * (5.0 / 9.0)
}

pub fn celsius_to_fahrenheit(cel: f32) -> f32 {
    (cel * 9.0 / 5.0) + 32.0
}
