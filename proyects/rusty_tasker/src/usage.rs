use colored::Colorize;

pub fn print_usage() {
    println!("Uso:".bold().yellow());
    println!("  add <description> <prioridad> <categoria> <tag1, tag2>   Agrega una tarea nueva");
    println!("              Filtros");
    println!("  lt                              Lista todas las tareas");
    println!("  lp                              Lista todas las tareas Pentientes");
    println!("  lc                              Lista todas las tareas Completadas");
    println!("--------------------------------------------------------------------");
    println!("  dt <id>                         Borra una tarea por ID");
    println!("  ed <id> <edita>                 Edita una tarea por ID");
    println!("  ct <id>                         Marca una tarea por ID");
    println!("  ut <id>                         Marca una tarea incompleta por ID");
}
