#[cfg(test)]

mod tests {
    use crate::tasks::{task::Priority, task_manager::TaskManager};

    //use super::*;
    use std::fs;

    #[test]
    fn test_save_tasks() {
        let mut task_manager = TaskManager::new();
        task_manager
            .add_task("Task1", Priority::High, Some("Work"), Some("tag1"))
            .unwrap();
        task_manager
            .add_task("Task2", Priority::Medium, Some("Personal"), Some("tag2"))
            .unwrap();

        let file_path = "test_save_tasks.json";
        let result = task_manager.save_tasks(file_path);

        assert!(result.is_ok(), "Se esperaba que save_tasks tenga exito");

        let file_content = fs::read_to_string(file_path).expect("Error al leer el fichero");
        let expected_task_json = serde_json::to_string(&task_manager.tasks).unwrap();
        assert_eq!(
            file_content, expected_task_json,
            "El contenido del fichero no coincide"
        );

        fs::remove_file(file_path).expect("Error al eliminar el fichero de prueba.")
    }
}
