use rand::{distr::StandardUniform, prelude::*};
use rand_chacha::ChaCha12Rng;
use std::cmp::{max, min};

use tracing::debug;

use super::notes::{Accidental, Note, NoteLetter};

pub trait NoteSequence: Send {
    fn next_note(&mut self) -> NoteTuple;
}

impl Distribution<Note> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Note {
        let letter: NoteLetter = rand::random();
        let accidental: Accidental = rand::random();
        let octave = rng.random_range(1..=4);

        Note {
            letter,
            accidental,
            octave,
        }
    }
}

impl Distribution<NoteLetter> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NoteLetter {
        let index = rng.random_range(0..=6);
        match index {
            0 => NoteLetter::C,
            1 => NoteLetter::D,
            2 => NoteLetter::E,
            3 => NoteLetter::F,
            4 => NoteLetter::G,
            5 => NoteLetter::A,
            6 => NoteLetter::B,
            _ => unreachable!(),
        }
    }
}

impl Distribution<Accidental> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Accidental {
        let index = rng.random_range(0..3);
        match index {
            0 => Accidental::Sharp,
            1 => Accidental::Flat,
            2 => Accidental::Natural,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteTuple {
    reference: Note,
    divergence: Note,
}

impl NoteTuple {
    fn try_new(ref_note: Note, div_note: Note) -> NoteTuple {
        let diff = (ref_note.to_midi_number() as i8 - div_note.to_midi_number() as i8).abs();
        debug!("Reference: {ref_note}, divergence: {div_note}. Difference: {diff}");

        NoteTuple {
            reference: ref_note,
            divergence: div_note,
        }
    }

    pub fn reference(&self) -> Note {
        self.reference
    }

    pub fn divergence(&self) -> Note {
        self.divergence
    }
}

pub struct RandomNoteSequence {
    rng: ChaCha12Rng,
}

impl RandomNoteSequence {
    pub fn new(rng: ChaCha12Rng) -> RandomNoteSequence {
        RandomNoteSequence { rng }
    }
}

impl NoteSequence for RandomNoteSequence {
    fn next_note(&mut self) -> NoteTuple {
        let ref_note: Note = self.rng.random();
        let octave: i8 = self.rng.random_range(-12..=12);

        let mut div_note = ref_note;
        let min_note = Note::new(NoteLetter::E, Accidental::Natural, 1);
        let max_note = Note::new(NoteLetter::F, Accidental::Natural, 4);

        if octave < 0 {
            div_note = max(div_note - octave.unsigned_abs(), min_note);
        } else {
            div_note = min(div_note + octave.unsigned_abs(), max_note);
        }

        NoteTuple::try_new(ref_note, div_note)
    }
}
