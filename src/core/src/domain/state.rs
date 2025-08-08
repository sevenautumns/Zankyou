use anyhow::{Result, bail};
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;

use crate::interfaces::ui::NoteGuess;

use super::{
    notes::Note,
    random::{NoteSequence, NoteTuple, RandomNoteSequence},
};

pub enum State {
    MainMenuState(MainMenuState),
    GameModeState(GameModeState),
}

impl Default for State {
    fn default() -> Self {
        State::MainMenuState(MainMenuState {})
    }
}

#[derive(Default)]
pub struct MainMenuState {}

pub enum GameModeState {
    RandomMode(Box<RandomMode>),
}

impl Default for GameModeState {
    fn default() -> Self {
        GameModeState::RandomMode(Box::default())
    }
}

pub struct RandomMode {
    note_generator: RandomNoteSequence,
    current_note_tuple: Option<NoteTuple>,
    _statistics: RandomModeStatistics,
}

impl Default for RandomMode {
    fn default() -> Self {
        let rng = ChaCha12Rng::from_os_rng();
        let note_generator = RandomNoteSequence::new(rng);
        Self {
            note_generator,
            current_note_tuple: None,
            _statistics: RandomModeStatistics,
        }
    }
}

impl RandomMode {
    pub fn next_note(&mut self) -> NoteTuple {
        let note_tuple = self.note_generator.next_note();
        self.current_note_tuple = Some(note_tuple.clone());
        note_tuple
    }

    pub fn note_guess(&mut self, note: Note) -> Result<NoteGuess> {
        match &self.current_note_tuple {
            Some(note_tuple) => Ok(NoteGuess::new(note_tuple.clone(), note)),
            None => bail!("Can not compare notes because no tuple was created first"),
        }
    }
}

//TODO: fill with content
// -> can be used for some adaptive learning approach
#[derive(Default)]
pub struct RandomModeStatistics;
