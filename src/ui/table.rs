use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, Row, Table},
};

use crate::App;

pub fn render_table(app: &mut App, area: Rect, frame: &mut Frame) {
    let header =
        Row::new(["NAME", "PID", "MEM", "USER"]).style(Style::new().add_modifier(Modifier::BOLD));

    let rows = app.procs.iter().map(|p| {
        Row::new([
            p.name.to_string(),
            p.pid.to_string(),
            format!("{} {}", p.mem.0, p.mem.1),
            p.user.to_string(),
        ])
    });

    let width = [
        Constraint::Length(8),
        Constraint::Fill(1),
        Constraint::Length(12),
        Constraint::Length(12),
    ];

    let table = Table::new(rows, width)
        .header(header)
        .column_spacing(2)
        .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .block(Block::bordered().title(" Processes "));

    frame.render_stateful_widget(table, area, &mut app.table_state);
}
