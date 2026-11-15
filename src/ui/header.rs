use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    widgets::{Block, Widget},
};

use crate::app::App;

pub fn render_header(app: &App, area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .border_set(border::THICK)
        .title_top(" Top3 ");

    // let inner_area = block.inner(area);
    block.render(area, buf)
}
