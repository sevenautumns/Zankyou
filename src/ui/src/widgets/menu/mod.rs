use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListState, StatefulWidget},
};

pub mod events;

use super::{DEFAULT_STYLE, HIGHLIGHT_STYLE, Selection};

pub struct MenuWidget {}

#[derive(Clone)]
pub struct SideMenuWidgetState {
    menu_items: Vec<String>,
    list_state: ListState,
    style: Style,
}

impl Selection for SideMenuWidgetState {
    fn select(&mut self, style: Style) {
        self.style = style;
    }
}

impl Default for SideMenuWidgetState {
    fn default() -> Self {
        Self {
            menu_items: vec!["Game".to_string(), "Config".to_string()],
            list_state: ListState::default().with_selected(Some(0)),
            style: DEFAULT_STYLE,
        }
    }
}

impl SideMenuWidgetState {
    fn next(&mut self) {
        if let Some(index) = self.list_state.selected() {
            let len = self.menu_items.len();
            if index.saturating_add(1) >= len {
                self.list_state.select(Some(0));
            } else {
                self.list_state.select_next();
            }
        }
    }

    fn previous(&mut self) {
        if let Some(index) = self.list_state.selected() {
            let len = self.menu_items.len();
            if index == 0 {
                self.list_state.select(Some(len));
            } else {
                self.list_state.select_previous();
            }
        }
    }
}

impl StatefulWidget for MenuWidget {
    type State = SideMenuWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let menu_block = Block::default()
            .title("Selection")
            .borders(Borders::ALL)
            .style(state.style);

        let menu_items = List::new(state.menu_items.clone())
            .block(menu_block)
            .gray()
            .highlight_style(HIGHLIGHT_STYLE);

        StatefulWidget::render(menu_items, area, buf, &mut state.list_state);
    }
}
