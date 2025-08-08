use std::time::Duration;

use crate::CoreModel;
use crate::UIMessageHandler;
use crate::domain::state::GameModeState;
use crate::domain::state::MainMenuState;
use crate::domain::state::State;
use crate::interfaces::ui::CoreGameMessage;
use crate::interfaces::ui::CoreMessage;
use crate::interfaces::ui::NextNoteTuple;
use crate::interfaces::ui::UIGameMessage;

impl UIMessageHandler for UIGameMessage {
    fn handle(self, model: &mut CoreModel) {
        match self {
            UIGameMessage::NoteRequest => {
                if let State::GameModeState(GameModeState::RandomMode(rm)) = &mut model.state {
                    let note_tuple = rm.next_note();
                    model
                        .ui
                        .send(CoreMessage::GameMessage(CoreGameMessage::NoteResponse(
                            NextNoteTuple::new(note_tuple.clone()),
                        )));

                    //TODO: move requests to another thread with some limits
                    model
                        .audio
                        .play_note(note_tuple.reference(), Duration::from_secs(1));
                    model
                        .audio
                        .play_note(note_tuple.divergence(), Duration::from_secs(1));
                }
                //TODO: handle unexpected case
            }
            UIGameMessage::StopRequest => {
                model.state = State::MainMenuState(MainMenuState::default())
            }
        }
    }
}
