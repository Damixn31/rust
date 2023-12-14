mod tasks {
    pub mod task;
    pub mod task_manager;
}

use tasks::task_manager::TaskManager;

fn main() {
    let mut task_manager = TaskManager::new();

    task_manager.add_task("Completar proyecto en Rust");
    task_manager.add_task("Serializacion y deserializacion con Serde");
    task_manager.add_task("Prioridad y fecha de vencimiento");
    task_manager.add_task("Interfaz de linea de comandos mejorada");
    task_manager.add_task("Colores y estilos en la salida");
    task_manager.add_task("Manejo de errores mejorados");
    task_manager.add_task("Test mas complejos");
    task_manager.add_task("Categorias y etiquetas");
    task_manager.add_task("Notificaciones");
    task_manager.add_task("Mejorar en la documentacion");
    task_manager.add_task("Loggin");
    task_manager.add_task("Reorganizacion del codigo");

    task_manager.save_tasks("tasks.json");

    // Crea un nuevo taskManager cargando tareas desde el fichero
    let mut loaded_task_manager = TaskManager::load_tasks("tasks.json");
    loaded_task_manager.add_task("cliente");

    println!("Tarea despues de cargar desde el fichero");
    loaded_task_manager.list_tasks();

    loaded_task_manager.save_tasks("tasks.json");

    task_manager.list_tasks();

    //task_manager.complete_task(1);

    //task_manager.list_tasks();
}
