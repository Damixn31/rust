use crate::helpers::table_helper::crate_table;
use crate::tasks::task::Task;
use crate::tasks::task::TaskError;
use serde::{Deserialize, Serialize};
use std::fs;

use super::task::Priority;
use colored::Colorize;
//use prettytable::{color, Attr};
use prettytable::{Cell, Row};

//use crate::helpers::table_helper::crate_table;

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
        let new_task = Task::new(id, description, priority, categories, tags)?;

        //if description.is_empty() {
        //    return Err(TaskError::EmptyDescription);
        //}
        self.tasks.push(new_task);

        Ok(())
    }

    // guarda tarea en el fichero
    pub fn save_tasks(&self, file_path: &str) {
        let tasks_json = serde_json::to_string(&self.tasks).expect("Error al serializar tarea");
        fs::write(file_path, tasks_json).expect("Error al escribir en el fichero");
    }

    // carga las tareas a fichera
    pub fn load_tasks(file_path: &str) -> Self {
        if let Ok(tasks_json) = fs::read_to_string(file_path) {
            if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&tasks_json) {
                return TaskManager { tasks };
            } else {
                eprintln!("Error al deserializar tareas desde el fichero JSON");
            }
        }
        TaskManager::new()
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
    pub fn complete_task(&mut self, task_id: u64) {
        for task in &mut self.tasks {
            if task.id == task_id {
                task.completed = true;
                break;
            }
        }
    }

    //marca como tarea pentiente
    pub fn uncomplete_task(&mut self, task_id: u64) {
        for task in &mut self.tasks {
            if task.id == task_id {
                task.completed = false;
                break;
            }
        }
    }
    // filtra todas las tareas
    pub fn list_tasks(&self) {
        let mut table = crate_table(vec![
            "Tarea",
            "Descripcion",
            "Prioridad",
            "Estado",
            "Categoria",
            "Tags",
            "Fecha de Creacion",
        ]);
        for task in &self.tasks {
            let status_colored = if task.completed {
                "Completada".green().to_string()
            } else {
                "Pentiente".red().to_string()
            };

            let categories_tasks = task.categories.as_deref().unwrap_or("N/A");
            let tags_tasks = task.tags.iter().cloned().collect::<Vec<_>>().join(", ");

            table.add_row(Row::new(vec![
                Cell::new(&task.id.to_string()),
                Cell::new(&task.description),
                Cell::new(&format!("{:?}", task.priority)),
                Cell::new(&status_colored),
                Cell::new(categories_tasks),
                Cell::new(&tags_tasks),
                Cell::new(&task.creation_time.to_string()),
            ]));
        }
        table.printstd();
    }

    // filtro por tareas pedientes
    pub fn list_pending_tasks(&self) {
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

        if pending_tasks.is_empty() {
            println!("No se encontro tareas pentientes.");
        } else {
            for task in pending_tasks {
                let status_colored = "Pentiente".red().to_string();
                let categories_tasks = task.categories.as_deref().unwrap_or("N/A");
                let tags_tasks = task.tags.iter().cloned().collect::<Vec<_>>().join(", ");

                table.add_row(Row::new(vec![
                    Cell::new(&task.id.to_string()),
                    Cell::new(&task.description),
                    Cell::new(&format!("{:?}", task.priority)),
                    Cell::new(&status_colored),
                    Cell::new(categories_tasks),
                    Cell::new(&tags_tasks),
                    Cell::new(&task.creation_time.to_string()),
                ]));
            }
            table.printstd();
        }
    }

    // filtro por tareas completadas
    pub fn list_complete_tasks(&self) {
        let complete_tasks: Vec<&Task> = self.tasks.iter().filter(|task| task.completed).collect();

        if complete_tasks.is_empty() {
            println!("No se encontro tareas completadas.");
        } else {
            println!("Tareas Completadas:");
            for task in complete_tasks {
                println!(
                    "ID: {}, Descripcion: {}, Priority: {:?}",
                    task.id, task.description, task.priority
                );
            }
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

            self.save_tasks("tasks.json");

            Ok(())
        } else {
            Err(format!("No se encontro ninguna tarea con el ID: {}", id))
        }
    }
}
