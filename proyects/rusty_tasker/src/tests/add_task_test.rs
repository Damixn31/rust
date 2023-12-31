#[cfg(test)]
mod test {
    //use chrono::Utc;

    use super::super::super::tasks::task::Priority;
    use super::super::super::tasks::task_manager::TaskManager;
    use std::collections::HashSet;

    #[test]
    fn test_add_task() {
        let mut task_manager = TaskManager::new();
        let description = "Test Task";
        let priority = Priority::High;
        let categories = Some("Work");
        let tags = Some("tag1,tag2");

        let cloned_priority = priority.clone();
        let result = task_manager.add_task(description, cloned_priority, categories, tags);

        assert!(result.is_ok(), "Se espera que add_task tenga exito!");
        assert_eq!(
            task_manager.tasks.len(),
            1,
            "Se esperaba que alla una tarea en la lista."
        );

        let added_task = &task_manager.tasks[0];
        assert_eq!(added_task.id, 1, "El ID de la tarea no es el esperado");
        assert_eq!(
            added_task.description, description,
            "La descripcion de la tarea no es la esperada"
        );
        assert_eq!(
            added_task.priority, priority,
            "La prioridad no es la esperada"
        );
        assert_eq!(
            added_task.categories,
            categories.map(|c| c.to_string()),
            "La categoria de la tarea no es la esperada"
        );

        let expected_tags: HashSet<String> = ["tag1".to_string(), "tag2".to_string()]
            .iter()
            .cloned()
            .collect();
        assert_eq!(
            added_task.tags, expected_tags,
            "Las etiquetas de las tareas no son las esperadas"
        );

        assert!(
            !added_task.completed,
            "Se esperaba que la tarea no este completada",
        );

        //let creation_time = Utc::now().to_rfc3339();
        //assert_eq!(
        //    added_task.creation_time, creation_time,
        //    "La fecha de creacion de la tarea no es la esperada.",
        //)
    }
}
