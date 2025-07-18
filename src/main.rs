use audio::AudioDevice;
use color_eyre::Result;
use logic::EventLoop;

pub mod audio;
pub mod interface;
pub mod logic;

fn main() -> Result<()> {
    colog::init();

    let audio = AudioDevice::new()?;
    let mut event_loop = EventLoop::new(audio);
    event_loop.run();
    Ok(())
}
