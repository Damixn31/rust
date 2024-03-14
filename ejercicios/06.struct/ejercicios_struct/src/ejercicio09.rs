// 9. Definir una estructura Empleado con campos para el nombre,
// el salario por hora y las horas trabajadas en la semana.
// Implementar un método para calcular el salario semanal del empleado.

pub struct Employee {
    pub name: String,
    pub salary_hour: f64,
    pub weekly_hours: f64,
}

impl Employee {
    pub fn new_employee(name: String, salary_hour: f64, weekly_hours: f64) -> Employee {
        Employee {
            name,
            salary_hour,
            weekly_hours,
        }
    }
    pub fn weekly_salary(&self) -> f64 {
        self.salary_hour * self.weekly_hours
    }
}
