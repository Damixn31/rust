use colored::Colorize;
use prettytable::{color, Attr, Cell, Row, Table};

use crate::tasks::task::{Task, TaskError};

pub fn crate_table(headers_labels: Vec<&str>) -> Table {
    let mut table = Table::new();
    let header_row: Vec<Cell> = headers_labels
        .iter()
        .map(|header| {
            Cell::new(header)
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE))
                .with_style(Attr::BackgroundColor(color::BRIGHT_BLACK))
        })
        .collect();
    table.add_row(Row::new(header_row));
    table
}

pub fn add_task_rows(table: &mut Table, tasks: &[&Task]) -> Result<(), TaskError> {
    for task in tasks {
        let status_colored = if task.completed {
            "Completada".green().to_string()
        } else {
            "Pendiente".red().to_string()
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
    Ok(())
}
