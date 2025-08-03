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
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time;
use tracing::error;

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

pub struct App {
    running: bool,
    msg: String,
}

impl App {
    fn new() -> App {
        App {
            running: true,
            msg: "Hello, world!".into(),
        }
    }
}

impl Drop for RatatuiView {
    fn drop(&mut self) {
        self.app.running = false;
    }
}

impl RatatuiView {
    pub fn new() -> (
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
                        CoreMessage::MainMenuMessage(_) => self.app.msg = "Current received message: Start".into(),
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
            .constraints([
                Constraint::Length(1),
                Constraint::Min(2),
                Constraint::Length(1),
            ])
            .split(area);
        let header = vertical_split[0];
        let body = vertical_split[1];
        let footer = vertical_split[2];

        let text = Text::raw("Zankyou");
        f.render_widget(text, header);
        let text = Text::raw(app.msg.clone());
        f.render_widget(text, body);
        let text = Text::raw("Press q to quit");
        f.render_widget(text, footer);
    }
}
