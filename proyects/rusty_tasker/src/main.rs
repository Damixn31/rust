use std::env;

mod tasks {
    pub mod task;
    pub mod task_manager;
}

use tasks::task_manager::TaskManager;

use crate::tasks::task::Priority;

fn print_usage() {
    println!("Uso:");
    println!("  add <description> <prioridad>   Agrega una tarea nueva");
    println!("              Filtros");
    println!("  lt                              Lista todas las tareas");
    println!("  lp                              Lista todas las tareas Pentientes");
    println!("  lc                              Lista todas las tareas Completadas");
    println!("--------------------------------------------------------------------");
    println!("  dt <id>                         Borra una tarea por ID");
    println!("  ed <id> <edita>                 Edita una tarea por ID");
    println!("  ct <id>                         Marca una tarea por ID");
    println!("  ut <id>              Marca una tarea incompleta por ID");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Uso: {} <commando>  [argumentos]", args[0]);
        return;
    }

    let command = &args[1];
    let arguments: Vec<&str> = args.iter().skip(2).map(|s| s.as_str()).collect();

    let mut loaded_task_manager = TaskManager::load_tasks("tasks.json");

    match command.as_str() {
        "h" => {
            print_usage();
        }
        "lt" => {
            loaded_task_manager.list_tasks();
        }
        "lp" => {
            loaded_task_manager.list_pending_tasks();
        }
        "lc" => {
            loaded_task_manager.list_complete_tasks();
        }
        "add" => {
            if let Some(description) = arguments.get(0) {
                let priority_str = arguments.get(1).unwrap_or(&"Medium");
                //let due_date = arguments.get(2);

                let priority = match *priority_str {
                    "Low" => Priority::Low,
                    "Medium" => Priority::Medium,
                    "High" => Priority::High,
                    _ => {
                        println!("Error: Prioridad no valida");
                        return;
                    }
                };

                println!("Valor de description:{}", description);

                let mut loaded_task_manager = TaskManager::load_tasks("tasks.json");

                loaded_task_manager.add_task(description, priority);

                loaded_task_manager.save_tasks("tasks.json");
                println!("Agregar tarea: {}", description);
            } else {
                println!("Error debes proporcionar una descriptcion de la tarea");
            }
        }
        "dl" => {
            if let Some(id_str) = arguments.get(0) {
                if let Ok(task_id) = id_str.parse::<u64>() {
                    loaded_task_manager.delete_task(task_id);
                    loaded_task_manager.save_tasks("tasks.json");
                    println!("Tarea eliminada con el ID: {}", task_id);
                } else {
                    println!("Error: El ID debe ser un numero entero");
                }
            } else {
                println!("Error: tenes que proporcinar un ID de la tarea que quieres eliminar");
            }
        }
        "ct" => {
            if let Some(id_str) = arguments.get(0) {
                if let Ok(id) = id_str.parse::<u64>() {
                    loaded_task_manager.complete_task(id);
                    loaded_task_manager.save_tasks("tasks.json")
                } else {
                    println!("Error: El iD debe ser un numero entero");
                }
            } else {
                println!("Error: tenes que proporcinar un ID de la tarea que queres marcar como completada");
            }
        }
        "ut" => {
            if let Some(id_str) = arguments.get(0) {
                if let Ok(id) = id_str.parse::<u64>() {
                    loaded_task_manager.uncomplete_task(id);
                    loaded_task_manager.save_tasks("tasks.json")
                } else {
                    println!("Error: El ID debe ser un numero");
                }
            } else {
                println!("Error: tenes que proporcinar un ID de la tarea que queres marcar como incompleta");
            }
        }
        "ed" => {
            if let Some(id_str) = arguments.get(0) {
                if let Ok(id) = id_str.parse::<u64>() {
                    if let Some(new_description) = arguments.get(1) {
                        let new_priority_str = arguments.get(2).unwrap_or(&"Medium");
                        let new_priority = match *new_priority_str {
                            "Low" => Priority::Low,
                            "Medium" => Priority::Medium,
                            "High" => Priority::High,
                            _ => {
                                println!("Error: Prioridad no valida.");
                                return;
                            }
                        };
                        match loaded_task_manager.edit_task(id, new_description, new_priority) {
                            Ok(()) => println!("Tarea editada con exito!"),
                            Err(err) => println!("Error al editar la tarea: {}", err),
                        }
                    } else {
                        println!(
                            "Error: tenes que proporcinar una nueva descriptcion para la tarea"
                        );
                    }
                } else {
                    println!("Error: El ID debe ser un numero entero");
                }
            } else {
                println!("Error: tenes que proporcinar el ID de la tarea a editar");
            }
        }

        _ => {
            println!("Commando no reconocido: {}", command);
        }
    }
}
