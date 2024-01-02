#[cfg(test)]

mod tests {

    use crate::tasks::{task::Priority, task_manager::TaskManager};

    #[test]
    fn test_complete_task() {
        let mut task_manager = TaskManager::new();

        let task_completed_id = 1;

        task_manager
            .add_task("Task1", Priority::High, Some("Work"), Some("tag1"))
            .unwrap();
        task_manager
            .add_task("Task2", Priority::Medium, Some("Personal"), Some("tag2"))
            .unwrap();

        let result = task_manager.complete_task(task_completed_id);

        assert!(result.is_ok(), "Se esperaba que complete_task tenga exito");

        let completed_task = task_manager
            .tasks
            .iter()
            .find(|task| task.id == task_completed_id);
        assert!(
            completed_task.map(|task| task.completed).unwrap_or(false),
            "Se esperaba que la tarea con el ID {} este completada",
            task_completed_id
        );
    }
}
