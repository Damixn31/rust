use colored::*;

pub fn print_usage() {
    println!("{}", "\n\t☞Uso:".bold().white());
    println!(
        "{}",
        "\tCOMANDOS \t\tARGUMENTOS \t\t\t\tDESCRIPCION"
            .bold()
            .cyan()
    );

    println!(
        "{}",
        "\t---------------------------------------------------------------------------------------------------"
            .bold()
            .red()
    );
    println!(
        "{}",
        "\tadd \t<description> <prioridad> <categoria> <tag1, tag2>\tAgrega una tarea nueva"
            .green()
    );
    //println!("{}", "\n\tFILTROS".bold().cyan());
    //println!("{}", "\t↓".cyan());
    println!(
        "{}",
        "\n\tlt \t\t\t\t\t\t\t\tLista todas las tareas".magenta()
    );
    println!(
        "{}",
        "\tlp  \t\t\t\t\t\t\t\tLista todas las tareas Pentientes"
            .bold()
            .magenta()
    );
    println!(
        "{}",
        "\tlc   \t\t\t\t\t\t\t\tLista todas las tareas Completadas".magenta()
    );
    println!("\n");
    println!("{}", "\tdt \t<id> \t\t\t\t\t\t\tBorra una tarea".red());
    println!(
        "{}",
        "\ted \t<id> <edita> \t\t\t\t\t\tEdita una tarea".white()
    );
    println!(
        "{}",
        "\tct \t<id> \t\t\t\t\t\t\tMarca una tarea completada".green()
    );
    println!(
        "{}",
        "\tut \t<id> \t\t\t\t\t\t\tMarca una tarea incompleta"
            .bold()
            .red()
    );
}
