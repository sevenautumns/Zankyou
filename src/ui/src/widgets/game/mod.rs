use core::{domain::random::NoteTuple, interfaces::ui::NoteGuess};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use super::Selection;

pub mod events;

pub struct GameWidget {}

#[derive(Default, Clone)]
pub struct GameWidgetState {
    style: Style,
    current_note_tuple: Option<NoteTuple>,
    current_note_guess: Option<NoteGuess>,
}

impl Selection for GameWidgetState {
    fn select(&mut self, style: Style) {
        self.style = style;
    }
}

impl GameWidgetState {
    pub fn set_note(&mut self, note_tuple: Option<NoteTuple>) {
        self.current_note_tuple = note_tuple
    }

    pub fn set_note_guess(&mut self, note_guess: Option<NoteGuess>) {
        self.current_note_guess = note_guess
    }

    pub fn reset(&mut self) {
        self.current_note_tuple = None;
        self.current_note_guess = None
    }
}

impl StatefulWidget for GameWidget {
    type State = GameWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let outer_block = Block::default()
            .title("Game")
            .borders(Borders::ALL)
            .style(state.style);

        let description = "Press n to play the next note!";
        let description_text = Text::raw(description);
        let mut note = "Current note: ".to_string();
        if let Some(note_tuple) = &state.current_note_tuple {
            note = format!("{}{}", note, note_tuple.reference());
        }
        let note_text = Text::raw(note.clone());

        let vertical_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(area);

        let horizontal_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(description.len() as u16),
                Constraint::Min(0),
            ])
            .split(vertical_split[1]);

        let horizontal_split2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(note.len() as u16),
                Constraint::Min(0),
            ])
            .split(vertical_split[2]);

        outer_block.render(area, buf);
        description_text.render(horizontal_split[1], buf);
        note_text.render(horizontal_split2[1], buf);
    }
}
