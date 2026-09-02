use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Buttons};

pub fn render_folder_widget(frame: &mut Frame, area: Rect, folder_paths: &[String], app: &mut App) {
    // number or rows until broken in a colum
    let max_columns: usize = 4;
    let box_height: u16 = 16;

    app.buttons.clear();

    let rows: Vec<&[String]> = folder_paths.chunks(max_columns).collect();

    let vertical_constraint: Vec<Constraint> = rows
        .iter()
        .map(|_| Constraint::Length(box_height))
        .collect();

    let row_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vertical_constraint)
        .split(area);

    for (row_idx, row_items) in rows.iter().enumerate() {
        let col_constraint = vec![Constraint::Ratio(1, max_columns as u32); max_columns];

        let col_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraint)
            .split(row_layout[row_idx]);

        for (col_idx, col_item) in row_items.iter().enumerate() {
            let folder_box = Paragraph::new(format!("🗂️ {}", col_item))
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(ratatui::style::Color::Cyan));

            frame.render_widget(folder_box, col_layout[col_idx]);

            app.buttons.push(Buttons {
                label: col_item.to_string(),
                area: col_layout[col_idx],
                value: col_item[1],
            });
            // app.buttons[btn_number].value = col_item;;
        }
    }
}
