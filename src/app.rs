use std::{io, time::Duration};

use crate::{
    action::Action,
    processes::{self, Proc},
    ui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, widgets::TableState};

pub struct App {
    pub exit: bool,
    pub procs: Vec<Proc>,
    pub table_state: TableState,
}

impl App {
    pub fn new() -> Self {
        let procs = processes::Proc::read_proc().unwrap();
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        Self {
            exit: false,
            procs,
            table_state,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        ui::render(self, frame);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                let action = self.map_key_to_action(key);
                self.update(action)
            }
        } else {
            self.update(Action::Tick);
        }
        Ok(())
    }

    fn map_key_to_action(&self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('j') => Action::Down,
            KeyCode::Char('k') => Action::Up,
            _ => Action::Tick,
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.exit = true,
            Action::Up => todo!(),
            Action::Down => todo!(),
            Action::Tick | Action::None => {}
        }
    }
}
