use std::env;
use std::u64;

use crate::tasks::task::Priority;
use crate::tasks::task::TaskError;

use crate::tasks::task_manager::TaskManager;
use crate::usage::menu::print_usage;

pub fn commands_arguments() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];
    let arguments: Vec<&str> = args.iter().skip(2).map(|s| s.as_str()).collect();

    let loaded_task_manager = TaskManager::load_tasks("tasks.json");

    match command.as_str() {
        "h" => {
            print_usage();
        }
        "lt" => match loaded_task_manager {
            Ok(task_manager) => match task_manager.list_tasks() {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Error al listar las tareas: {:?}", err)
                }
            },
            Err(err) => {
                eprintln!("Error al listar tareas: {:?}", err);
            }
        },
        "lp" => match loaded_task_manager {
            Ok(task_manager) => match task_manager.list_pending_tasks() {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Error al listar las tareas pentientes: {:?}", err)
                }
            },
            Err(err) => {
                eprintln!("Error al listar las tareas tareas: {:?}", err)
            }
        },

        "lc" => match loaded_task_manager {
            Ok(task_manager) => match task_manager.list_complete_tasks() {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("Error al listar las tareas completadas: {:?}", err)
                }
            },
            Err(err) => {
                eprintln!("Error al listar las tareas: {:?}", err)
            }
        },
        "add" => match (arguments.first(), arguments.get(1)) {
            (Some(description), Some(priority_str)) => {
                let priority = match priority_str.to_lowercase().as_str() {
                    "low" => Priority::Low,
                    "medium" => Priority::Medium,
                    "high" => Priority::High,
                    _ => {
                        eprintln!("Error: prioridad no valida");
                        return;
                    }
                };

                let tags = arguments.get(3).map(|tags_str| tags_str.trim());
                let categories: Option<&str> =
                    arguments.get(2).map(|category_str| category_str.trim());

                match loaded_task_manager {
                    Ok(mut task_manager) => {
                        if let Err(err) =
                            task_manager.add_task(description, priority, categories, tags)
                        {
                            eprintln!("Error al agregar la tarea: {:?}", err);
                            return;
                        }
                        if let Err(err) = task_manager.save_tasks("tasks.json") {
                            eprintln!("Error al guardar la tarea: {:?}", err);
                        }
                    }

                    Err(TaskError::EmptyDescription) => {
                        eprintln!("Error la descripcion no puede estar vacia.");
                    }
                    _ => {
                        eprintln!("Error al agregar tarea");
                    }
                }
            }
            _ => {
                eprintln!("Error: tenes que proporcinar una descripcion y prioridad para la tarea");
            }
        },
        "dt" => {
            if let Some(id_str) = arguments.first() {
                if let Ok(task_id) = id_str.parse::<u64>() {
                    match loaded_task_manager {
                        Ok(mut task_manager) => match task_manager.delete_task(task_id) {
                            Ok(()) => {
                                if let Err(err) = task_manager.save_tasks("tasks.json") {
                                    eprintln!("Error al guardar: {:?}", err);
                                }
                            }
                            Err(err) => {
                                eprintln!("Error la tarea que intentas borrar no exite: {:?}", err);
                            }
                        },
                        Err(err) => {
                            eprintln!("Error al borrar la tarea: {:?}", err);
                        }
                    }
                } else {
                    println!("Error: El ID debe ser un número entero");
                }
            } else {
                println!("Error: Debes proporcionar un ID de la tarea que quieres eliminar");
            }
        }
        "ct" => {
            if let Some(id_str) = arguments.first() {
                if let Ok(id) = id_str.parse::<u64>() {
                    match loaded_task_manager {
                        Ok(mut task_manager) => match task_manager.complete_task(id) {
                            Ok(()) => {
                                if let Err(err) = task_manager.save_tasks("tasks.json") {
                                    eprintln!("Error al guardar: {:?}", err);
                                }
                            }
                            Err(err) => {
                                eprintln!("Error al marcar la tarea completada: {:?}", err);
                            }
                        },
                        Err(err) => {
                            eprintln!("Error al marcar la tarea completada: {:?}", err);
                        }
                    }
                } else {
                    println!("Error: El ID debe ser un número entero");
                }
            } else {
                println!(
                    "Error: Debes proporcionar un ID de la tarea que quieres marcar como completa"
                );
            }
        }
        "ut" => {
            if let Some(id_str) = arguments.first() {
                if let Ok(id) = id_str.parse::<u64>() {
                    match loaded_task_manager {
                        Ok(mut task_manager) => match task_manager.uncomplete_task(id) {
                            Ok(()) => {
                                if let Err(err) = task_manager.save_tasks("tasks.json") {
                                    eprintln!("Error al guardar: {:?}", err);
                                }
                            }
                            Err(err) => {
                                eprintln!("Error al marcar la tarea pendiente: {:?}", err);
                            }
                        },
                        Err(err) => {
                            eprintln!("Error al marcar la tarea pentientes: {:?}", err);
                        }
                    }
                } else {
                    println!("Error: El ID debe ser un número entero");
                }
            } else {
                println!(
                    "Error: Debes proporcionar un ID de la tarea que quieres marcar como pentiente"
                );
            }
        }
        "ed" => {
            if let Some(id_str) = arguments.first() {
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

                        match loaded_task_manager {
                            Ok(mut task_manager) => {
                                match task_manager.edit_task(id, new_description, new_priority) {
                                    Ok(()) => {}
                                    Err(err) => {
                                        eprintln!("Error al editar: {:?}", err);
                                    }
                                }
                            }
                            Err(err) => {
                                eprintln!("Error al editar: {:?}", err);
                            }
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
        "dd" => match loaded_task_manager {
            Ok(mut task_manager) => match task_manager.clear_tasks() {
                Ok(()) => {
                    if let Err(err) = task_manager.save_tasks("tasks.json") {
                        eprintln!("Error al guardar: {:?}", err);
                    }
                }

                Err(err) => {
                    eprintln!("Error algo salio mal: {:?}", err);
                }
            },
            Err(err) => {
                eprintln!("Error al borrar todas las tareas: {:?}", err);
            }
        },

        _ => {
            println!("Commando no reconocido: {}", command);
        }
    }
}
