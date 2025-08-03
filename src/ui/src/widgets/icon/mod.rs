use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::Text,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

pub struct IconWidget {}

#[derive(Default, Clone)]
pub struct IconWidgetState {}

impl StatefulWidget for IconWidget {
    type State = IconWidgetState;

    //TODO: render some icon
    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
        let outer_block = Block::default().title("").borders(Borders::ALL).gray();
        let icon = Paragraph::new(Text::raw("Zankyou")).block(outer_block);
        icon.render(area, buf);
    }
}
