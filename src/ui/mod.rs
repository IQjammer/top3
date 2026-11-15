pub mod header;
use crate::App;
use ratatui::{buffer::Buffer, layout::Rect};

pub fn render(app: &App, area: Rect, buf: &mut Buffer) {
    header::render_header(app, area, buf)
}
