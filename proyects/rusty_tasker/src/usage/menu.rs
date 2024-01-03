use colored::*;

pub fn print_usage() {
    macro_rules! print_line {
        ($desc:expr, $color:ident) => {
            println!("{:<10} {:<30} {:<50}", $desc.0.bold(), $desc.1, $desc.2);
        };
    }

    let arg_use = format!("\n\t☞Uso:{}", "".green());
    println!("{}", arg_use);
    print_line!(
        (
            "\tCOMANDOS".cyan(),
            "\t\tARGUMENTOS".bright_blue(),
            "\t\tDESCRIPCION".magenta()
        ),
        cyan
    );
    print_line!(
        ("\t-------------------------------------------------------------------------------------------", "", ""),
        cyan
    );
    print_line!(
        (
            "\tadd",
            "<description> <prioridad> <categoria> <tag1, tag2>",
            "\tAgrega una tarea nueva".green()
        ),
        green
    );
    print_line!(
        ("\tlt", "", "\t\t\tLista todas las tareas".bright_yellow()),
        magenta
    );
    print_line!(
        ("\tlp", "", "\t\t\tLista tareas pendientes".bright_yellow()),
        red
    );
    print_line!(
        ("\tlc", "", "\t\t\tLista tareas completadas".bright_yellow()),
        green
    );
    println!("\n");
    print_line!(("\tdt", "<id>", "\t\t\tBorra una tarea".red()), red);
    print_line!(
        ("\ted", "<id> <edita>", "\t\t\tEdita una tarea".cyan()),
        white
    );
    print_line!(
        (
            "\tct",
            "<id>",
            "\t\t\tMarca una tarea completada".bright_green()
        ),
        green
    );
    print_line!(
        (
            "\tut",
            "<id>",
            "\t\t\tMarca una tarea incompleta".bright_red()
        ),
        red
    );
    print_line!(
        ("\tdd", "", "\t\t\tBorra todas las tareas".red().bold()),
        red
    );
}
