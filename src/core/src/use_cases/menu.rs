use crate::CoreModel;
use crate::UIMessageHandler;
use crate::domain::state::GameModeState;
use crate::domain::state::State;
use crate::interfaces::ui::UIMainMenuMessage;

impl UIMessageHandler for UIMainMenuMessage {
    fn handle(self, model: &mut CoreModel) {
        match self {
            UIMainMenuMessage::Start => {
                model.state = State::GameModeState(GameModeState::default());
                //TODO: start listener
            }
            UIMainMenuMessage::Quit => model.running = false,
        }
    }
}
