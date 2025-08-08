use ratatui::style::{Color, Style};

pub mod config;
pub mod game;
pub mod icon;
pub mod menu;

pub const DEFAULT_STYLE: Style = Style::new().fg(Color::Gray);
pub const HIGHLIGHT_STYLE: Style = Style::new().fg(Color::Cyan);

pub trait Selection {
    fn select(&mut self, style: Style);
    fn unselect(&mut self) {
        self.select(DEFAULT_STYLE);
    }
}
