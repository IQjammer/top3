use std::{io, time::Duration};

use crate::{action::Action, ui};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

pub struct App {
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { exit: false }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        ui::render(self, frame.area(), frame.buffer_mut());
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
