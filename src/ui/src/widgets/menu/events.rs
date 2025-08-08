use core::interfaces::ui::{CoreMainMenuMessage, UIMainMenuMessage, UserInterfaceMessage};

use crossterm::event::{Event, KeyCode};

use crate::{
    CoreMessageHandler, CursorMain, CursorMenu, CursorState, EventHandler, MenuConfig, MenuGame,
    MenuState,
};

impl CoreMessageHandler for CoreMainMenuMessage {
    fn handle(self, _view: &mut crate::RatatuiView) {
        todo!()
    }
}

impl EventHandler for CursorMenu {
    fn handle(self, event: Event, view: &mut crate::RatatuiView) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char('q') => {
                    view.core_interface
                        .send(UserInterfaceMessage::MainMenuMessage(
                            core::interfaces::ui::UIMainMenuMessage::Quit,
                        ));
                }
                _ => {
                    let menu_state = view.app.menu_state.clone();
                    menu_state.handle(event, view);
                }
            }
        }
    }
}

impl EventHandler for MenuState {
    fn handle(self, event: Event, view: &mut crate::RatatuiView) {
        match self {
            MenuState::Game(game) => game.handle(event, view),
            MenuState::Config(config) => config.handle(event, view),
        }
    }
}

impl EventHandler for MenuGame {
    fn handle(self, event: Event, view: &mut crate::RatatuiView) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Right | KeyCode::Enter => {
                    view.core_interface
                        .send(UserInterfaceMessage::MainMenuMessage(
                            UIMainMenuMessage::Start,
                        ));
                    view.app.transition_cursor(CursorState::Main(CursorMain {}));
                }
                KeyCode::Up => {
                    view.app.transition_menu(MenuState::Config(MenuConfig {}));
                    view.app.menu_widget.previous();
                }
                KeyCode::Down => {
                    view.app.transition_menu(MenuState::Config(MenuConfig {}));
                    view.app.menu_widget.next();
                }
                _ => {}
            }
        }
    }
}

impl EventHandler for MenuConfig {
    fn handle(self, event: Event, view: &mut crate::RatatuiView) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Right | KeyCode::Enter => {
                    view.app
                        .transition_cursor(crate::CursorState::Main(CursorMain {}));
                }
                KeyCode::Up => {
                    view.app.transition_menu(MenuState::Game(MenuGame {}));
                    view.app.menu_widget.previous();
                }
                KeyCode::Down => {
                    view.app.transition_menu(MenuState::Game(MenuGame {}));
                    view.app.menu_widget.next();
                }
                _ => {}
            }
        }
    }
}
