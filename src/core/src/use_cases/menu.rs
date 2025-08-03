use crate::CoreMessageHandler;
use crate::CoreModel;
use crate::interfaces::ui::CoreMainMenuMessage;
use crate::interfaces::ui::CoreMessage;
use crate::interfaces::ui::UIMainMenuMessage;

impl CoreMessageHandler for UIMainMenuMessage {
    fn handle(self, model: &mut CoreModel) {
        match self {
            UIMainMenuMessage::Start => model
                .ui
                .send(CoreMessage::MainMenuMessage(CoreMainMenuMessage::Start)),
            UIMainMenuMessage::Quit => model.running = false,
        }
    }
}
