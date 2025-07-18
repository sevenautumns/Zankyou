use std::{
    ops::{Add, Sub},
    time::Duration,
};

pub trait AudioInterface {
    fn play_note(&mut self, note: Note, interval: Duration);
}

// Define an enum for the letter name of the note
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum NoteLetter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

// Define accidentals
#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub enum Accidental {
    Sharp,
    Flat,
    #[default]
    Natural,
}

// Define the struct for a musical note
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Note {
    pub letter: NoteLetter,
    pub accidental: Option<Accidental>, // Optional accidental
    pub octave: u8,
}

impl Note {
    /// Creates a new Note.
    pub fn new(letter: NoteLetter, accidental: Option<Accidental>, octave: u8) -> Self {
        Note {
            letter,
            accidental,
            octave,
        }
    }

    /// Converts the Note to a MIDI note number (0-127).
    /// Middle C (C4) is typically MIDI note 60.
    /// This assumes A4 = 440Hz, and C4 is the C below A4.
    pub fn to_midi_number(&self) -> u8 {
        // Base MIDI note for C0 is 12 (C0 to B0 are 12-23)
        // C0 is MIDI 12, C1 is MIDI 24, C2 is MIDI 36, C3 is MIDI 48, C4 is MIDI 60, etc.

        let mut midi_number: i16 = match self.letter {
            NoteLetter::C => 0,
            NoteLetter::D => 2,
            NoteLetter::E => 4,
            NoteLetter::F => 5,
            NoteLetter::G => 7,
            NoteLetter::A => 9,
            NoteLetter::B => 11,
        };

        // Add octave contribution
        // MIDI notes start from C-1 (MIDI note 0). C0 is MIDI 12. C4 is MIDI 60.
        // So (octave + 1) * 12 gives the base for the C of that octave.
        midi_number += ((self.octave as i16) + 1) * 12;

        // Adjust for accidental
        if let Some(acc) = self.accidental {
            match acc {
                Accidental::Sharp => midi_number += 1,
                Accidental::Flat => midi_number -= 1,
                Accidental::Natural => {} // Natural doesn't change the base pitch
            }
        }

        // Clamp to MIDI range (0-127)
        midi_number.clamp(0, 127) as u8
    }

    /// Converts a MIDI note number back to a Note.
    /// This is an approximation and might not perfectly preserve accidentals
    /// (e.g., it won't distinguish C# from Db unless more sophisticated logic is added).
    /// For simplicity, it will prefer sharps for # pitches and natural for natural pitches.
    pub fn from_midi_number(midi_number: u8) -> Self {
        let midi_number_i16 = midi_number as i16;

        let octave = ((midi_number_i16 / 12) - 1) as u8; // Deduce octave
        let pitch_class = midi_number_i16 % 12; // Deduce pitch class (0=C, 1=C#, ...)

        let (letter, accidental) = match pitch_class {
            0 => (NoteLetter::C, None),
            1 => (NoteLetter::C, Some(Accidental::Sharp)), // C#
            2 => (NoteLetter::D, None),
            3 => (NoteLetter::D, Some(Accidental::Sharp)), // D#
            4 => (NoteLetter::E, None),
            5 => (NoteLetter::F, None),
            6 => (NoteLetter::F, Some(Accidental::Sharp)), // F#
            7 => (NoteLetter::G, None),
            8 => (NoteLetter::G, Some(Accidental::Sharp)), // G#
            9 => (NoteLetter::A, None),
            10 => (NoteLetter::A, Some(Accidental::Sharp)), // A#
            11 => (NoteLetter::B, None),
            _ => unreachable!(), // Should not happen with modulo 12
        };

        Note {
            letter,
            accidental,
            octave,
        }
    }
}

// Implement Add trait for Note + u8 (semitones)
impl Add<u8> for Note {
    type Output = Note;

    fn add(self, semitones: u8) -> Self::Output {
        let current_midi = self.to_midi_number();
        let new_midi = current_midi.saturating_add(semitones); // Use saturating_add to prevent overflow past 127
        Note::from_midi_number(new_midi)
    }
}

// Implement Sub trait for Note - u8 (semitones)
impl Sub<u8> for Note {
    type Output = Note;

    fn sub(self, semitones: u8) -> Self::Output {
        let current_midi = self.to_midi_number();
        // Use saturating_sub to prevent underflow below 0
        let new_midi = current_midi.saturating_sub(semitones);
        Note::from_midi_number(new_midi)
    }
}
