use async_trait::async_trait;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tracing::error;

#[async_trait]
pub trait UserInterfaceTrait: std::fmt::Debug + Send {
    async fn receive(&mut self) -> UserInterfaceMessage;
    fn send(&mut self, message: CoreMessage);
}

#[derive(Debug)]
pub struct UserInterface {
    sender: UnboundedSender<CoreMessage>,
    receiver: UnboundedReceiver<UserInterfaceMessage>,
}

impl UserInterface {
    pub fn new() -> (
        Self,
        UnboundedSender<UserInterfaceMessage>,
        UnboundedReceiver<CoreMessage>,
    ) {
        let (core_sender, core_receiver) = unbounded_channel();
        let (ui_sender, ui_receiver) = unbounded_channel();
        let ui = Self {
            sender: core_sender,
            receiver: ui_receiver,
        };

        (ui, ui_sender, core_receiver)
    }
}

#[async_trait]
impl UserInterfaceTrait for UserInterface {
    async fn receive(&mut self) -> UserInterfaceMessage {
        self.receiver.recv().await.expect("infinite stream from ui")
    }

    fn send(&mut self, message: CoreMessage) {
        if let Err(err) = self.sender.send(message) {
            error!(?err, "Failed to send core message to ui");
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserInterfaceMessage {
    MainMenuMessage(UIMainMenuMessage),
    GameMessage(UIGameMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UIMainMenuMessage {
    Start,
    Quit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UIGameMessage {
    Play,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreMessage {
    MainMenuMessage(CoreMainMenuMessage),
    GameMessage(CoreGameMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreMainMenuMessage {
    Start,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreGameMessage;
