use crate::helpers::table_helper::add_task_rows;
use crate::helpers::table_helper::crate_table;
use crate::tasks::task::Task;
use crate::tasks::task::TaskError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::result::Result;

use super::task::Priority;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager { tasks: Vec::new() }
    }

    // agrega tarea
    pub fn add_task(
        &mut self,
        description: &str,
        priority: Priority,
        categories: Option<&str>,
        tags: Option<&str>,
    ) -> Result<(), TaskError> {
        let id = (self.tasks.len() + 1) as u64;

        // Convertir tags de &str a HashSet<String>
        let tags_set: HashSet<String> = match tags {
            Some(tags_str) => tags_str
                .split(',')
                .map(|tag| tag.trim().to_string())
                .collect(),
            None => HashSet::new(),
        };

        let clone_priority = priority.clone();
        let new_task = Task::new(id, description, clone_priority, categories, Some(tags_set))?;

        self.tasks.push(new_task);

        Ok(())
    }

    // guarda tarea en el fichero
    pub fn save_tasks(&self, file_path: &str) -> Result<(), TaskError> {
        let tasks_json = serde_json::to_string(&self.tasks).map_err(|err| {
            TaskError::SerializationError(format!("Error al serializar tarea: {}", err))
        })?;
        fs::write(file_path, tasks_json).map_err(|err| {
            TaskError::FileWriteError(format!("Error al escribir el fichero: {}", err))
        })?;
        Ok(())
    }

    // carga las tareas al fichero
    pub fn load_tasks(file_path: &str) -> Result<Self, TaskError> {
        if let Ok(tasks_json) = fs::read_to_string(file_path) {
            if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&tasks_json) {
                return Ok(TaskManager { tasks });
            } else {
                eprintln!("Error al deserializar tareas desde el fichero JSON");
            }
        }
        Ok(TaskManager::new())
    }

    // borra tarea
    pub fn delete_task(&mut self, id: u64) -> Result<(), TaskError> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(TaskError::TaskNotFound)
        }
    }

    // marca como tarea completa
    pub fn complete_task(&mut self, task_id: u64) -> Result<(), TaskError> {
        for task in &mut self.tasks {
            if task.id == task_id {
                task.completed = true;
                break;
            }
        }
        Ok(())
    }

    //marca como tarea pentiente
    pub fn uncomplete_task(&mut self, task_id: u64) -> Result<(), TaskError> {
        for task in &mut self.tasks {
            if task.id == task_id {
                task.completed = false;
                break;
            }
        }
        Ok(())
    }
    // lista todas las tareas
    pub fn list_tasks(&self) -> Result<(), TaskError> {
        let mut table = crate_table(vec![
            "Tarea",
            "Descripcion",
            "Prioridad",
            "Estado",
            "Categoria",
            "Tags",
            "Fecha de Creacion",
        ]);
        let task_refs: Vec<&Task> = self.tasks.iter().collect();
        add_task_rows(&mut table, &task_refs)?;

        table.printstd();
        Ok(())
    }

    // lista por tareas pedientes
    pub fn list_pending_tasks(&self) -> Result<(), TaskError> {
        let mut table = crate_table(vec![
            "Tarea",
            "Descripcion",
            "Prioridad",
            "Estado",
            "Categorias",
            "Tags",
            "Fecha de Creacion",
        ]);

        let pending_tasks: Vec<&Task> = self.tasks.iter().filter(|task| !task.completed).collect();

        add_task_rows(&mut table, &pending_tasks)?;

        table.printstd();

        Ok(())
    }

    // Lista las tareas completadas
    pub fn list_complete_tasks(&self) -> Result<(), TaskError> {
        // filtro por tareas completadas
        let complete_tasks: Vec<&Task> = self.tasks.iter().filter(|task| task.completed).collect();

        if complete_tasks.is_empty() {
            println!("No se encontron las tareas completadas");
            Ok(())
        } else {
            let mut table = crate_table(vec![
                "Tarea",
                "Descripcion",
                "Prioridad",
                "Estado",
                "Categorias",
                "Tags",
                "Fecha de Creacion",
            ]);
            add_task_rows(&mut table, &complete_tasks)?;
            table.printstd();
            Ok(())
        }
    }

    // edita descripcion de la tarea
    pub fn edit_task(
        &mut self,
        id: u64,
        new_description: &str,
        new_priority: Priority,
    ) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.description = new_description.to_string();
            task.priority = new_priority;

            if let Err(err) = self.save_tasks("tasks.json") {
                eprintln!("Error al guardar la tarea: {:?}", err)
            }

            Ok(())
        } else {
            Err(format!("No se encontro ninguna tarea con el ID: {}", id))
        }
    }

    pub fn clear_tasks(&mut self) -> Result<(), TaskError> {
        self.tasks.clear();
        Ok(())
    }
}
