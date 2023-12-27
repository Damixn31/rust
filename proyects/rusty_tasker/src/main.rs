mod tasks {
    pub mod task;
    pub mod task_manager;
}

mod usage {
    pub mod commands;
    pub mod menu;
}

use usage::commands::commands_arguments;

fn main() {
    commands_arguments()
}
