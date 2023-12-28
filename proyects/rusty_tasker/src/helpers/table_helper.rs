use prettytable::{color, Attr, Cell, Row, Table};

pub fn crate_table(headers: Vec<&str>) -> Table {
    let mut table = Table::new();
    let header_row: Vec<Cell> = headers
        .iter()
        .map(|header| {
            Cell::new(header)
                //.with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(color::BRIGHT_WHITE))
                .with_style(Attr::BackgroundColor(color::BRIGHT_BLACK))
        })
        .collect();
    table.add_row(Row::new(header_row));
    table
}
