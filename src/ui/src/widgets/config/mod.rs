use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::Text,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

pub struct ConfigWidget {}

#[derive(Default, Clone)]
pub struct ConfigWidgetState {}

impl StatefulWidget for ConfigWidget {
    type State = ConfigWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        let outer_block = Block::default()
            .title("Config")
            .borders(Borders::ALL)
            .gray();

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Length(1),
                Constraint::Percentage(49),
            ])
            .split(area);
        let placeholder = Text::raw("Hello, World!");

        outer_block.render(area, buf);
        placeholder.render(vertical_chunks[1], buf);
    }
}
