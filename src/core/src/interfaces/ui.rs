use async_trait::async_trait;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tracing::error;

use crate::domain::{notes::Note, random::NoteTuple};

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
    NoteRequest,
    StopRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreMessage {
    MainMenuMessage(CoreMainMenuMessage),
    GameMessage(CoreGameMessage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreGameMessage {
    NoteResponse(NextNoteTuple),
    GuessResponse(NoteGuess),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NextNoteTuple {
    pub note_tuple: NoteTuple,
}

impl NextNoteTuple {
    pub fn new(note_tuple: NoteTuple) -> Self {
        Self { note_tuple }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteGuess {
    pub true_note_tuple: NoteTuple,
    pub note_played: Note,
    pub correct: bool,
    pub score: u8,
}

impl NoteGuess {
    pub fn new(note_tuple: NoteTuple, note: Note) -> Self {
        let correct = note_tuple.reference().eq(&note);
        Self {
            true_note_tuple: note_tuple.clone(),
            note_played: note,
            correct,
            score: note_tuple.divergence().distance(&note),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreMainMenuMessage;
