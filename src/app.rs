use std::{
    io,
    time::{Duration, Instant},
};

use crate::{
    action::Action,
    processes::{self, Proc},
    ui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, widgets::TableState};

const REFRESH: Duration = Duration::from_millis(300);

pub struct App {
    pub exit: bool,
    pub procs: Vec<Proc>,
    pub table_state: TableState,
    pub last_refresh: Instant,
}

impl App {
    pub fn new() -> Self {
        let mut procs = processes::Proc::read_proc().unwrap();
        procs.sort_unstable_by_key(|p| std::cmp::Reverse(p.mem));
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        Self {
            exit: false,
            procs,
            table_state,
            last_refresh: Instant::now(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = REFRESH.saturating_sub(self.last_refresh.elapsed());

            if event::poll(timeout)? {
                self.handle_events()?;
            }

            if self.last_refresh.elapsed() >= REFRESH {
                self.procs = Proc::read_proc().unwrap_or_default();
                self.procs
                    .sort_unstable_by_key(|p| std::cmp::Reverse(p.mem));
                self.last_refresh = Instant::now()
            }
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
            Action::Up => self.table_state.select_previous(),
            Action::Down => self.table_state.select_next(),
            Action::Tick => {}
        }
    }
}
