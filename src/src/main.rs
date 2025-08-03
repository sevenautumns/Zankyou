use anyhow::Result;
use audio::AudioDevice;
use core::Core;
use ui::RatatuiView;

#[tokio::main]
async fn main() -> Result<()> {
    let audio = AudioDevice::new()?;
    let (ui, handle) = RatatuiView::create();
    let mut core = Core::new(Box::new(audio), Box::new(ui));
    tokio::task::spawn(async move { core.run().await });
    handle.await
}
