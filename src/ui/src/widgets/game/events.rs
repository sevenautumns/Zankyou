use core::interfaces::ui::CoreGameMessage;

use crate::CoreMessageHandler;

impl CoreMessageHandler for CoreGameMessage {
    fn handle(self, view: &mut crate::RatatuiView) {
        match self {
            CoreGameMessage::NoteResponse(note_tuple) => {
                view.app.game_widget.set_note(Some(note_tuple.note_tuple));
            }
            CoreGameMessage::GuessResponse(note_guess) => {
                view.app.game_widget.set_note_guess(Some(note_guess));
            }
        }
    }
}
