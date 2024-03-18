#[derive(Debug)]
pub struct UsageInfo {
    program_name: String,
}

impl UsageInfo {
    pub fn new(program_name: &str) -> Self {
        UsageInfo {
            program_name: program_name.to_string(),
        }
    }

    pub fn print_usege(&self) {
        println!("Uso: {} <directorio_local>", self.program_name);
        //println!("Argumentos Opcionales:");
    }
}
