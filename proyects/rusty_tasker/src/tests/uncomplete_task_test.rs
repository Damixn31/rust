#[cfg(test)]

mod tests {
    use crate::tasks::{task::Priority, task_manager::TaskManager};

    #[test]
    fn test_uncomplete_task() {
        let mut task_manager = TaskManager::new();
        let uncomplete_task_id = 1;

        task_manager
            .add_task("Task1", Priority::High, Some("Work"), Some("tag1"))
            .unwrap();
        task_manager
            .add_task("Task2", Priority::Medium, Some("Personal"), Some("tag2"))
            .unwrap();

        task_manager
            .uncomplete_task(uncomplete_task_id)
            .expect("Error al completar la tarea");

        let result = task_manager.uncomplete_task(uncomplete_task_id);

        assert!(
            result.is_ok(),
            "Se esperaba que uncomplete_task tenga exito"
        );

        let uncompleted_task = task_manager
            .tasks
            .iter()
            .find(|task| task.id == uncomplete_task_id);
        assert!(
            uncompleted_task
                .map(|task| !task.completed)
                .unwrap_or(false),
            "Se esperaba que la tarea con el id {} este pendiente",
            uncomplete_task_id
        );
    }
}
