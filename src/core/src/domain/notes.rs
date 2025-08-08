use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Sub},
};

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

impl fmt::Display for NoteLetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            NoteLetter::C => 'C',
            NoteLetter::D => 'D',
            NoteLetter::E => 'E',
            NoteLetter::F => 'F',
            NoteLetter::G => 'G',
            NoteLetter::A => 'A',
            NoteLetter::B => 'B',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
pub enum Accidental {
    Sharp,
    Flat,
    #[default]
    Natural,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Note {
    pub letter: NoteLetter,
    pub accidental: Accidental,
    pub octave: u8,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let acc = match self.accidental {
            Accidental::Sharp => "#",
            Accidental::Flat => "b",
            Accidental::Natural => "",
        };
        write!(f, "{}{}{}", self.letter, acc, self.octave)
    }
}

impl Note {
    pub fn new(letter: NoteLetter, accidental: Accidental, octave: u8) -> Self {
        Note {
            letter,
            accidental,
            octave,
        }
    }

    // FIXME
    pub fn to_midi_number(&self) -> u8 {
        let mut midi_number: i16 = match self.letter {
            NoteLetter::C => 0,
            NoteLetter::D => 2,
            NoteLetter::E => 4,
            NoteLetter::F => 5,
            NoteLetter::G => 7,
            NoteLetter::A => 9,
            NoteLetter::B => 11,
        };
        midi_number += ((self.octave as i16) + 1) * 12;

        match self.accidental {
            Accidental::Sharp => midi_number += 1,
            Accidental::Flat => midi_number -= 1,
            Accidental::Natural => {}
        }
        midi_number.clamp(0, 127) as u8
    }

    pub fn from_midi_number(midi_number: u8) -> Self {
        let midi_number_i16 = midi_number as i16;

        let octave = ((midi_number_i16 / 12) - 1) as u8;
        let pitch_class = midi_number_i16 % 12;

        let (letter, accidental) = match pitch_class {
            0 => (NoteLetter::C, Accidental::Natural),
            1 => (NoteLetter::C, Accidental::Sharp),
            2 => (NoteLetter::D, Accidental::Natural),
            3 => (NoteLetter::D, Accidental::Sharp),
            4 => (NoteLetter::E, Accidental::Natural),
            5 => (NoteLetter::F, Accidental::Natural),
            6 => (NoteLetter::F, Accidental::Sharp),
            7 => (NoteLetter::G, Accidental::Natural),
            8 => (NoteLetter::G, Accidental::Sharp),
            9 => (NoteLetter::A, Accidental::Natural),
            10 => (NoteLetter::A, Accidental::Sharp),
            11 => (NoteLetter::B, Accidental::Natural),
            _ => unreachable!(),
        };

        Note {
            letter,
            accidental,
            octave,
        }
    }

    pub fn distance(&self, note: &Note) -> u8 {
        let note_value = self.to_midi_number();
        let other_note_value = note.to_midi_number();

        if note_value > other_note_value {
            note_value.saturating_sub(other_note_value)
        } else {
            other_note_value.saturating_sub(note_value)
        }
    }
}

impl Add<u8> for Note {
    type Output = Note;

    fn add(self, semitones: u8) -> Self::Output {
        let current_midi = self.to_midi_number();
        let new_midi = current_midi.saturating_add(semitones);
        Note::from_midi_number(new_midi)
    }
}

impl Sub<u8> for Note {
    type Output = Note;

    fn sub(self, semitones: u8) -> Self::Output {
        let current_midi = self.to_midi_number();
        let new_midi = current_midi.saturating_sub(semitones);
        Note::from_midi_number(new_midi)
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_midi_number().cmp(&other.to_midi_number())
    }
}
