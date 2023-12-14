use crate::tasks::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: &str) {
        let id = (self.tasks.len() + 1) as u64;
        let task = Task::new(id, description);
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, task_id: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.complete();
        } else {
            print!("No se encontro la tarea  con ID {}", task_id);
        }
    }

    pub fn list_tasks(&self) {
        println!("Lista de tareas:");
        for task in &self.tasks {
            let status = if task.completed {
                "Completada"
            } else {
                "Pendiente"
            };
            println!(
                "ID: {}, descripcion: {}, Estado: {}",
                task.id, task.description, status
            );
        }
        println!();
    }
}
