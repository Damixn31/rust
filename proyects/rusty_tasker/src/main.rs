mod tasks {
    pub mod task;
    pub mod task_manager;
}

use tasks::task_manager::TaskManager;

fn main() {
    let mut task_manager = TaskManager::new();

    task_manager.add_task("Completar proyecto en Rust");
    task_manager.add_task("Preparar la comida");

    task_manager.list_tasks();

    task_manager.complete_task(1);

    task_manager.list_tasks();
}
