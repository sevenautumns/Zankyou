use std::time::Duration;

use crate::domain::notes::Note;

pub trait AudioInterfaceTrait: std::fmt::Debug + Send {
    fn play_note(&mut self, note: Note, interval: Duration);
}
