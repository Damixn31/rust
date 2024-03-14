// 7. Definir una Estructura 'CuentaBancaria' con campos nombre del titular, el numero de cuenta y
//    el saldo. Implementar metodos para depositar, retirar, consultar saldo.

pub struct BankAccount {
    pub headline_name: String,
    pub number_account: String,
    pub balance: f64,
}

impl BankAccount {
    pub fn new_account(headline_name: String, number_account: String, balance: f64) -> BankAccount {
        BankAccount {
            headline_name,
            number_account,
            balance,
        }
    }
    pub fn deposit(&mut self, amount: f64) {
        // aca no tenemos que un tipo de retorno porque
        // nosotros queremos agregar
        self.balance += amount
    }

    pub fn extract(&mut self, amount: f64) -> Result<(), &'static str> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Saldo insuficiente para realizar el retiro.")
        }
    }

    pub fn check_balance(&self) -> f64 {
        self.balance
    }
}
