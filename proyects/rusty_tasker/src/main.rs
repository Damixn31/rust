mod tasks {
    pub mod task;
    pub mod task_manager;
}

mod usage {
    pub mod commands;
    pub mod menu;
}

mod helpers {
    pub mod table_helper;
}

mod tests {
    mod add_task_test;
    mod complete_task_test;
    mod delete_task_test;
    mod load_tasks_test;
    mod save_tasks_test;
    mod uncomplete_task_test;
}

use usage::commands::commands_arguments;

fn main() {
    commands_arguments()
}
