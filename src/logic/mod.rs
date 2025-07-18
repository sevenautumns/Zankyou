use log::debug;
use rand::{distr::StandardUniform, prelude::*};
use std::{
    cmp::{max, min},
    io::{self, Write, stdout},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self},
    time::Duration,
};

use crate::interface::{Accidental, AudioInterface, Note, NoteLetter};

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

pub trait NoteSequence {
    fn next_note(&mut self) -> NoteTuple;
}

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
}

pub struct RandomNoteSequence<R: RngCore> {
    rng: R,
}

impl<R: RngCore> RandomNoteSequence<R> {
    fn new(rng: R) -> RandomNoteSequence<R> {
        RandomNoteSequence { rng }
    }
}

impl<R: RngCore> NoteSequence for RandomNoteSequence<R> {
    fn next_note(&mut self) -> NoteTuple {
        let ref_note: Note = self.rng.random();
        let octave: i8 = self.rng.random_range(-12..=12);

        let mut div_note = ref_note;
        let min_note = Note::new(NoteLetter::E, Accidental::Natural, 1);
        let max_note = Note::new(NoteLetter::F, Accidental::Natural, 4);

        if octave < 0 {
            div_note = max(div_note - octave.abs() as u8, min_note);
        } else {
            div_note = min(div_note + octave.abs() as u8, max_note);
        }

        NoteTuple::try_new(ref_note, div_note)
    }
}

pub struct EventLoop {
    audio: Box<dyn AudioInterface>,
}

impl EventLoop {
    pub fn new<A: AudioInterface + 'static>(audio: A) -> EventLoop {
        EventLoop {
            audio: Box::new(audio),
        }
    }
}

impl EventLoop {
    pub fn run(&mut self) {
        let rng = rand::rng();
        let mut rng_notes = RandomNoteSequence::new(rng);

        loop {
            let note_tuple = rng_notes.next_note();
            self.wait_next();
            self.play_note(note_tuple.reference, note_tuple.divergence);
        }
    }

    fn wait_next(&mut self) {
        println!("Press Enter to proceed to the next note");
        stdout().flush().unwrap();

        loop {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();

            if buf.trim().is_empty() {
                break;
            }
        }
    }

    fn play_note(&mut self, ref_note: Note, div_note: Note) {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = stop.clone();

        thread::spawn(move || {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();

            let line = buf.trim();
            if line.is_empty() {
                stop_clone.store(true, Ordering::Relaxed);
            }
        });

        while !stop.load(Ordering::Relaxed) {
            println!("Playing first note: {ref_note}");
            stdout().flush().unwrap();
            self.audio.play_note(ref_note, Duration::from_secs(1));

            println!("Playing second note");
            stdout().flush().unwrap();
            self.audio.play_note(div_note, Duration::from_secs(2));
        }

        println!("Second note was: {div_note}");
        stdout().flush().unwrap();
    }
}
