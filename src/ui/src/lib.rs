use core::interfaces::ui::{CoreMessage, UIGameMessage, UserInterface, UserInterfaceMessage};
use std::io::{self, Stdout};
use std::pin::Pin;
use std::time::Duration;

use anyhow::{Context, Result};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::event::{EventStream, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use futures::StreamExt;
use gag::Gag;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time;
use tracing::error;
use widgets::config::{ConfigWidget, ConfigWidgetState};
use widgets::game::{GameWidget, GameWidgetState};
use widgets::icon::{IconWidget, IconWidgetState};
use widgets::menu::{MenuWidget, SideMenuWidgetState};
use widgets::{HIGHLIGHT_STYLE, Selection};

mod widgets;

// handles messages from the core
pub trait CoreMessageHandler {
    fn handle(self, view: &mut RatatuiView);
}

// handles keyboard/mouse events
pub trait EventHandler {
    fn handle(self, event: Event, view: &mut RatatuiView);
}

impl CoreMessageHandler for CoreMessage {
    fn handle(self, view: &mut crate::RatatuiView) {
        match self {
            CoreMessage::MainMenuMessage(msg) => msg.handle(view),
            CoreMessage::GameMessage(msg) => msg.handle(view),
        }
    }
}

pub struct RatatuiView {
    app: App,
    core_interface: CoreInterface,
}

pub struct CoreInterface {
    sender: UnboundedSender<UserInterfaceMessage>,
    receiver: UnboundedReceiver<CoreMessage>,
}

impl CoreInterface {
    fn new(
        sender: UnboundedSender<UserInterfaceMessage>,
        receiver: UnboundedReceiver<CoreMessage>,
    ) -> Self {
        Self { sender, receiver }
    }
}

impl CoreInterface {
    async fn receive(&mut self) -> CoreMessage {
        self.receiver
            .recv()
            .await
            .expect("infinite stream from core")
    }

    fn send(&mut self, message: UserInterfaceMessage) {
        if let Err(err) = self.sender.send(message) {
            error!(?err, "Failed to send ui message to core");
        }
    }
}

//TODO: clean up the states
#[derive(Debug, Clone)]
pub enum MenuState {
    Game(MenuGame),
    Config(MenuConfig),
}

impl Default for MenuState {
    fn default() -> Self {
        Self::Game(MenuGame {})
    }
}

#[derive(Default, Debug, Clone)]
pub struct MenuGame {}

#[derive(Default, Debug, Clone)]
pub struct MenuConfig {}

#[derive(Debug, Clone)]
pub enum CursorState {
    Menu(CursorMenu),
    Main(CursorMain),
}

impl Default for CursorState {
    fn default() -> Self {
        Self::Menu(CursorMenu {})
    }
}

#[derive(Default, Debug, Clone)]
pub struct CursorMenu {}

#[derive(Default, Debug, Clone)]
pub struct CursorMain {}

impl EventHandler for CursorState {
    fn handle(self, event: Event, view: &mut RatatuiView) {
        match self {
            CursorState::Menu(menu) => menu.handle(event, view),
            CursorState::Main(main) => main.handle(event, view),
        }
    }
}

impl EventHandler for CursorMain {
    fn handle(self, event: Event, view: &mut crate::RatatuiView) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Left | KeyCode::Esc => {
                    view.app.transition_cursor(CursorState::Menu(CursorMenu {}));
                    view.app.game_widget.reset();
                }
                _ => match view.app.menu_state {
                    MenuState::Game(_) => if let KeyCode::Char('n') = key_event.code { view.core_interface.send(
                        UserInterfaceMessage::GameMessage(UIGameMessage::NoteRequest),
                    ) },
                    MenuState::Config(_) => {}
                },
            }
        }
    }
}

pub struct App {
    icon_widget: IconWidgetState,
    menu_widget: SideMenuWidgetState,
    game_widget: GameWidgetState,
    config_widget: ConfigWidgetState,
    cursor_state: CursorState,
    menu_state: MenuState,
    running: bool,
}

impl App {
    fn new() -> App {
        let mut menu_widget = SideMenuWidgetState::default();
        menu_widget.select(Style::default().fg(Color::Cyan));
        App {
            icon_widget: IconWidgetState::default(),
            menu_widget,
            game_widget: GameWidgetState::default(),
            config_widget: ConfigWidgetState::default(),
            cursor_state: CursorState::default(),
            menu_state: MenuState::default(),
            running: true,
        }
    }

    fn transition_cursor(&mut self, state: CursorState) {
        match (&self.cursor_state, &self.menu_state) {
            (CursorState::Menu(_), _) => self.menu_widget.unselect(),
            (CursorState::Main(_), MenuState::Game(_)) => self.game_widget.unselect(),
            (CursorState::Main(_), MenuState::Config(_)) => self.config_widget.unselect(),
        }

        match (&state, &self.menu_state) {
            (CursorState::Menu(_), _) => self.menu_widget.select(HIGHLIGHT_STYLE),
            (CursorState::Main(_), MenuState::Game(_)) => self.game_widget.select(HIGHLIGHT_STYLE),
            (CursorState::Main(_), MenuState::Config(_)) => {
                self.config_widget.select(HIGHLIGHT_STYLE)
            }
        }

        self.cursor_state = state;
    }

    fn transition_menu(&mut self, menu: MenuState) {
        self.menu_state = menu;
    }
}

impl Drop for RatatuiView {
    fn drop(&mut self) {
        self.app.running = false;
    }
}

impl RatatuiView {
    pub fn create() -> (
        UserInterface,
        Pin<Box<dyn Future<Output = anyhow::Result<()>>>>,
    ) {
        let (ui, ui_sender, core_receiver) = UserInterface::new();
        let core_interface = CoreInterface::new(ui_sender, core_receiver);
        let app = App::new();
        let mut view = Self {
            app,
            core_interface,
        };
        let handle = Box::pin(async move { view.run().await });

        (ui, handle)
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut terminal = Self::setup_terminal().expect("tui setup failed");
        let mut event = EventStream::new();
        let mut tick_rate = time::interval(Duration::from_millis(50));
        let _suppress_stderr = Gag::stderr().unwrap();

        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic| {
            Self::restore_terminal().ok();
            original_hook(panic);
        }));

        let mut needs_update = false;
        terminal.draw(|f| Self::render(f, &mut self.app))?;

        while self.app.running {
            tokio::select! {
                event = self.core_interface.receive() => {
                    event.handle(self);
                    needs_update = true;
                }
                ct_event = event.next() => {
                    self.handle_event(ct_event).await;
                    needs_update = true;
                },
                _ = tick_rate.tick() => {
                    if needs_update {
                        terminal.draw(|f| Self::render(f, &mut self.app))?;
                        needs_update = false;
                    }
                }
            }
        }

        Self::restore_terminal()
    }

    fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
        enable_raw_mode().context("failed to enable raw mode")?;
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)
            .context("unable to enter alternate screen")?;
        let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))
            .context("failed to create terminal")?;
        terminal.hide_cursor().context("failed to hide cursor")?;
        Ok(terminal)
    }

    fn restore_terminal() -> Result<()> {
        disable_raw_mode().context("failed to disable raw mode")?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    async fn handle_event(&mut self, ct_event: Option<Result<Event, std::io::Error>>) {
        if let Some(Ok(event)) = ct_event {
            let cursor_event = self.app.cursor_state.clone();
            cursor_event.handle(event, self);
        }
    }

    fn render(f: &mut Frame, app: &mut App) {
        let area = f.area();
        let vertical_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(area);

        let horizontal_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(30), Constraint::Min(10)])
            .split(vertical_split[0]);

        let side_menu = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(2)])
            .split(horizontal_split[0]);

        let footer_block = Block::default()
            .borders(Borders::NONE)
            .gray()
            .padding(Padding::new(1, 0, 0, 0));

        let quit_text = Paragraph::new(Text::raw("Press q to quit")).block(footer_block);
        f.render_stateful_widget(IconWidget {}, side_menu[0], &mut app.icon_widget);
        f.render_stateful_widget(MenuWidget {}, side_menu[1], &mut app.menu_widget);
        f.render_widget(quit_text, vertical_split[1]);

        match app.menu_state {
            MenuState::Game(_) => {
                f.render_stateful_widget(GameWidget {}, horizontal_split[1], &mut app.game_widget)
            }
            MenuState::Config(_) => f.render_stateful_widget(
                ConfigWidget {},
                horizontal_split[1],
                &mut app.config_widget,
            ),
        }
    }
}
