mod action;
mod app;
mod processes;
mod ui;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    println!("{:?}", processes::Proc::read_proc());
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let app_result = app.run(&mut terminal);

    ratatui::restore();

    app_result
}
