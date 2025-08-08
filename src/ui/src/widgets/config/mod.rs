use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use super::Selection;

pub struct ConfigWidget {}

#[derive(Default, Clone)]
pub struct ConfigWidgetState {
    style: Style,
}

impl Selection for ConfigWidgetState {
    fn select(&mut self, style: Style) {
        self.style = style;
    }
}

impl StatefulWidget for ConfigWidget {
    type State = ConfigWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let outer_block = Block::default()
            .title("Config")
            .borders(Borders::ALL)
            .style(state.style);

        let vertical_split = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
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

        let placeholder = Text::raw("Config selected");

        outer_block.render(area, buf);
        placeholder.render(horizontal_split[1], buf);
    }
}
