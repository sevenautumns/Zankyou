use domain::state::State;
use interfaces::{audio::AudioInterfaceTrait, ui::UserInterfaceTrait};
use tracing::debug;

pub mod domain;
pub mod interfaces;
pub mod use_cases;

pub trait CoreMessageHandler {
    fn handle(self, model: &mut CoreModel);
}

pub struct CoreModel {
    audio: Box<dyn AudioInterfaceTrait>,
    ui: Box<dyn UserInterfaceTrait>,
    state: State,
    running: bool,
}

pub struct Core {
    model: CoreModel,
}

impl Core {
    pub fn new(audio: Box<dyn AudioInterfaceTrait>, ui: Box<dyn UserInterfaceTrait>) -> Self {
        Self {
            model: CoreModel {
                audio,
                ui,
                state: State::default(),
                running: true,
            },
        }
    }
}

impl Core {
    pub async fn run(&mut self) {
        while self.model.running {
            tokio::select! {
                ui_event = self.model.ui.receive() => {
                    debug!(?ui_event, "Received ui event");
                    match ui_event {
                        interfaces::ui::UserInterfaceMessage::MainMenuMessage(msg) => msg.handle(&mut self.model),
                        interfaces::ui::UserInterfaceMessage::GameMessage(msg) => msg.handle(&mut self.model),
                    }
                }
            }
        }
    }
}
