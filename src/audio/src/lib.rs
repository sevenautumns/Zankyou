use core::{domain::notes::Note, interfaces::audio::AudioInterfaceTrait};
use std::{io::Cursor, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use cpal::{
    Device, FromSample, Host, I24, SampleFormat, SizedSample, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use tracing::error;

pub struct AudioDevice {
    _host: Host,
    output: Device,
    stream_config: StreamConfig,
    synth: Synthesizer,
    output_format: SampleFormat,
}

const SOUNDFONT: &[u8] = include_bytes!("./bass.sf2");

impl std::fmt::Debug for AudioDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioDevice").finish()
    }
}

impl AudioDevice {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let output = host
            .default_output_device()
            .context("Couldn't obtain default output device")?;
        let stream_config = output.default_output_config()?;
        let output_format = stream_config.sample_format();

        let sample_rate = stream_config.sample_rate();
        let synth_settings = SynthesizerSettings::new(sample_rate.0 as i32);

        let soundfont = SoundFont::new(&mut Cursor::new(SOUNDFONT))?;
        let soundfont = Arc::new(soundfont);
        let synth = Synthesizer::new(&soundfont, &synth_settings)?;
        Ok(Self {
            _host: host,
            output,
            synth,
            output_format,
            stream_config: stream_config.into(),
        })
    }

    fn generate_waveform(&mut self, note: Note, interval: Duration) -> Waveform {
        let sample_count = interval.as_secs() as usize * self.synth.get_sample_rate() as usize;
        let mut left: Vec<f32> = vec![0_f32; sample_count];
        let mut right: Vec<f32> = vec![0_f32; sample_count];

        self.synth.note_off_all(true);
        self.synth.note_on(0, note.to_midi_number() as i32, 100);
        self.synth.render(&mut left[..], &mut right[..]);

        Waveform {
            left,
            _right: right,
        }
    }
}

impl AudioInterfaceTrait for AudioDevice {
    fn play_note(&mut self, note: Note, interval: Duration) {
        let waveform = self.generate_waveform(note, interval);
        waveform.play(self).unwrap();
    }
}

struct Waveform {
    left: Vec<f32>,
    _right: Vec<f32>,
}

impl Waveform {
    fn play(&self, device: &AudioDevice) -> Result<()> {
        match device.output_format {
            SampleFormat::I8 => self.play_inner::<i8>(device),
            SampleFormat::I16 => self.play_inner::<i16>(device),
            SampleFormat::I24 => self.play_inner::<I24>(device),
            SampleFormat::I32 => self.play_inner::<i32>(device),
            SampleFormat::I64 => self.play_inner::<i64>(device),
            SampleFormat::U8 => self.play_inner::<u8>(device),
            SampleFormat::U32 => self.play_inner::<u32>(device),
            SampleFormat::U64 => self.play_inner::<u64>(device),
            SampleFormat::F32 => self.play_inner::<f32>(device),
            SampleFormat::F64 => self.play_inner::<f64>(device),
            _ => unimplemented!(),
        }
    }

    fn play_inner<T>(&self, device: &AudioDevice) -> Result<()>
    where
        T: SizedSample + FromSample<f32>,
    {
        let duration_secs = self.left.len() as u64 / device.stream_config.sample_rate.0 as u64;
        let duration = Duration::from_secs(duration_secs);

        let mono = self.left.clone();
        let mut index = 0;
        let mut next_value = move || {
            let value = mono[index];
            index += 1;
            index %= mono.len();
            value
        };

        let channels = device.stream_config.channels as usize;
        let err_fn = |err| error!("an error occured on stream: {err}");
        let data_callback = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let next = next_value();
                let value = T::from_sample(next);
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        };
        let stream = device.output.build_output_stream(
            &device.stream_config,
            data_callback,
            err_fn,
            None,
        )?;
        stream.play()?;

        std::thread::sleep(duration);

        Ok(())
    }
}
