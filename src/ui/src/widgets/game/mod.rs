use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Text,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

pub struct GameWidget {}

#[derive(Default, Clone)]
pub struct GameWidgetState {}

impl StatefulWidget for GameWidget {
    type State = GameWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        let outer_block = Block::default().title("Game").borders(Borders::ALL).gray();

        let vertical_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(49),
            ])
            .split(area);

        let horizontal_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Length(14),
                Constraint::Percentage(49),
            ])
            .split(vertical_split[1]);

        let placeholder = Text::raw("Hello, World!");

        outer_block.render(area, buf);
        placeholder.render(horizontal_split[1], buf);
    }
}
