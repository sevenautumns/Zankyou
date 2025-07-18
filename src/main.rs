use std::time::Duration;

use audio::AudioDevice;
use color_eyre::Result;
use interface::{AudioInterface, Note};

pub mod audio;
pub mod interface;
pub mod logic;

fn main() -> Result<()> {
    colog::init();

    let mut audio = AudioDevice::new()?;
    audio.play_note(
        Note::new(interface::NoteLetter::C, None, 3),
        Duration::from_secs(1),
    );
    audio.play_note(
        Note::new(interface::NoteLetter::E, None, 3),
        Duration::from_secs(1),
    );
    audio.play_note(
        Note::new(interface::NoteLetter::G, None, 3),
        Duration::from_secs(1),
    );

    Ok(())
}
