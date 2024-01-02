#[cfg(test)]

mod tests {
    use crate::tasks::{task::Priority, task_manager::TaskManager};
    use std::fs;

    #[test]
    fn test_load_tasks() {

        let mut task_manager = TaskManager::new();
        task_manager.add_task("Task 1", Priority::High, Some("Work"), Some("tag1")).unwrap();
        task_manager.add_task("Task 2", Priority::Medium, Some("Personal"), Some("tag2")).unwrap();
        

        let file_path = "test_load_tasks.json";
        task_manager.save_tasks(file_path).expect("Error guardar las tareas en el fichero");

        let loaded_task_manager = TaskManager::load_tasks(file_path).expect("Error al cargar las tareas desde el fichero");

        assert_eq!(loaded_task_manager.tasks.len(), task_manager.tasks.len(), "La cantidad de tareas no coinciden");
        
        for (loaded_task, expected_task) in loaded_task_manager.tasks.iter().zip(task_manager.tasks.iter()) {
            assert_eq!(loaded_task.id, expected_task.id, "El ID de la tarea no coincide");
            assert_eq!(loaded_task.description, expected_task.description, "La descripción de la tarea no coincide");
        }

        fs::remove_file(file_path).expect("Error al eliminar el fichero de prueba");

    }
}
