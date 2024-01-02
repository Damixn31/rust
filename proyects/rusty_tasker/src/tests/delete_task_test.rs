#[cfg(test)]

mod tests {
    use crate::tasks::{task::Priority, task_manager::TaskManager};

    #[test]
    fn test_detele_task() {
        let mut task_manager = TaskManager::new();
        let id_delete = 1;
        task_manager
            .add_task("Task1", Priority::High, Some("Work"), Some("tag1"))
            .unwrap();
        task_manager
            .add_task("Task2", Priority::High, Some("Personal"), Some("tag2"))
            .unwrap();

        let result = task_manager.delete_task(id_delete);

        assert!(result.is_ok(), "Se esperaba que delete_task tenga exito");

        assert_eq!(
            task_manager.tasks.iter().find(|task| task.id == id_delete),
            None,
            "Se esperaba que la tarea con el ID {} haya sido eliminada",
            id_delete
        );
    }
}
