pub mod table;
use crate::App;
use ratatui::Frame;

pub fn render(mut app: &mut App, frame: &mut Frame) {
    let area = frame.area();
    table::render_table(&mut app, area, frame);
}
