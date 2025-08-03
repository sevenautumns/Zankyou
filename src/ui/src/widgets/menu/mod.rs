use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, ListState, StatefulWidget},
};

use crate::MenuState;

pub struct MenuWidget {}

#[derive(Clone)]
pub struct SideMenuWidgetState {
    _menu_state: MenuState,
    menu_items: Vec<String>,
    list_state: ListState,
}

impl Default for SideMenuWidgetState {
    fn default() -> Self {
        Self {
            _menu_state: MenuState::default(),
            menu_items: vec!["Game".to_string(), "Config".to_string()],
            list_state: ListState::default(),
        }
    }
}

impl StatefulWidget for MenuWidget {
    type State = SideMenuWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let menu_block = Block::default()
            .title("Selection")
            .borders(Borders::ALL)
            .gray();

        let menu_items = List::new(state.menu_items.clone())
            .block(menu_block)
            .highlight_style(Style::default().fg(Color::Cyan));

        StatefulWidget::render(menu_items, area, buf, &mut state.list_state);
    }
}
