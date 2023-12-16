use crate::tasks::task::Task;
use serde::{Deserialize, Serialize};
use std::fs;

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
    pub fn add_task(&mut self, description: &str, priority: Priority) {
        let id = (self.tasks.len() + 1) as u64;
        let new_task = Task::new(id, description, priority);
        self.tasks.push(new_task);
    }

    // guarda tarea en el fichero
    pub fn save_tasks(&self, file_path: &str) {
        let tasks_json = serde_json::to_string(&self.tasks).expect("Error al serializar tarea");
        fs::write(file_path, tasks_json).expect("Error al escribir en el fichero");
    }

    // carga las tareas a fichero
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
    pub fn delete_task(&mut self, task_id: u64) {
        self.tasks.retain(|task| task.id != task_id);
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

    // filtro por tareas pedientes
    pub fn list_pending_tasks(&self) {
        let pending_tasks: Vec<&Task> = self.tasks.iter().filter(|task| !task.completed).collect();

        if pending_tasks.is_empty() {
            println!("No se encontro tareas pentientes.");
        } else {
            println!("Tareas Pendientes:");
            for task in pending_tasks {
                println!(
                    "ID: {}, Descripcion: {}, Prioridad: {:?}",
                    task.id, task.description, task.priority
                );
            }
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
