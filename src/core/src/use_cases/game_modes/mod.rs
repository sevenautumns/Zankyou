use std::time::Duration;

use crate::CoreMessageHandler;
use crate::CoreModel;
use crate::domain::random::NoteSequence;
use crate::domain::random::RandomNoteSequence;
use crate::interfaces::ui::UIGameMessage;

impl CoreMessageHandler for UIGameMessage {
    fn handle(self, model: &mut CoreModel) {
        match self {
            UIGameMessage::Play => {
                let rng = rand::rng();
                let mut rng_note_gen = RandomNoteSequence::new(rng);
                let note_tuple = rng_note_gen.next_note();
                model
                    .audio
                    .play_note(note_tuple.reference(), Duration::from_secs(1));
            }
        }
    }
}
