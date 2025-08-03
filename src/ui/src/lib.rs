use core::interfaces::ui::{
    CoreMessage, UIGameMessage, UIMainMenuMessage, UserInterface, UserInterfaceMessage,
};
use std::io::{self, Stdout};
use std::pin::Pin;
use std::time::Duration;

use anyhow::{Context, Result};
use crossterm::event::EventStream;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
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

mod widgets;

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

#[derive(Default, Debug, Clone)]
pub enum MenuState {
    #[default]
    Game,
    Config,
}

#[derive(Default, Debug, Clone)]
pub enum CursorState {
    #[default]
    Menu,
    Main,
}

pub struct App {
    icon_widget: IconWidgetState,
    menu_widget: SideMenuWidgetState,
    game_widget: GameWidgetState,
    config_widget: ConfigWidgetState,
    _cursor_state: CursorState,
    menu_state: MenuState,
    running: bool,
}

impl App {
    fn new() -> App {
        App {
            icon_widget: IconWidgetState::default(),
            menu_widget: SideMenuWidgetState::default(),
            game_widget: GameWidgetState::default(),
            config_widget: ConfigWidgetState::default(),
            _cursor_state: CursorState::default(),
            menu_state: MenuState::default(),
            running: true,
        }
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
                    match event {
                        CoreMessage::MainMenuMessage(_) => todo!(),
                        CoreMessage::GameMessage(_) => todo!()
                    }
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
        if let Some(Ok(Event::Key(event))) = ct_event {
            match event.code {
                crossterm::event::KeyCode::Enter => {
                    self.core_interface
                        .send(UserInterfaceMessage::MainMenuMessage(
                            UIMainMenuMessage::Start,
                        ));
                }
                crossterm::event::KeyCode::Char('q') => {
                    self.core_interface
                        .send(UserInterfaceMessage::MainMenuMessage(
                            UIMainMenuMessage::Quit,
                        ));
                }
                crossterm::event::KeyCode::Char('n') => {
                    self.core_interface
                        .send(UserInterfaceMessage::GameMessage(UIGameMessage::Play));
                }
                _ => {}
            }
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
            MenuState::Game => {
                f.render_stateful_widget(GameWidget {}, horizontal_split[1], &mut app.game_widget)
            }
            MenuState::Config => f.render_stateful_widget(
                ConfigWidget {},
                horizontal_split[1],
                &mut app.config_widget,
            ),
        }
    }
}
