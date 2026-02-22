use std::f32::consts::PI;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ratatui::{DefaultTerminal, Frame};

pub struct NoteSequence {
    notes: Vec<f32>,
    bpm: f32,
    looped: bool,
}

impl NoteSequence {
    pub fn bpm(mut self, bpm: u32) -> Self {
        self.bpm = bpm as f32;
        self
    }

    pub fn looped(mut self) -> Self {
        self.looped = true;
        self
    }

    pub fn play(self) -> Result<cpal::Stream, anyhow::Error> {
        if self.notes.is_empty() {
            anyhow::bail!("NoteSequence cannot be played");
        }

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");
        let supported_config = device.default_output_config()?;
        let sample_rate = supported_config.sample_rate() as f32;
        let channels = supported_config.channels();
        let config: cpal::StreamConfig = supported_config.into();

        let note_duration = 0.5;
        let samples_per_note = (sample_rate * note_duration) as usize;

        let mut sample_index = 0usize;
        let mut note_index = 0usize;
        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                for frame in data.chunks_mut(channels as usize) {
                    let freq = self.notes[note_index];

                    let t = sample_index as f32 / sample_rate;
                    let value = (2.0 * PI * freq * t).sin() * 0.2;

                    for sample in frame {
                        *sample = value;
                    }

                    sample_index += 1;

                    if sample_index >= samples_per_note {
                        sample_index = 0;
                        note_index = (note_index + 1) % self.notes.len();
                    }
                }
            },
            |err| eprintln!("an error occurred on output stream: {}", err),
            None,
        )?;

        stream.play()?;

        Ok(stream)
    }
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("hello world", frame.area());
}

fn main() -> color_eyre::Result<()> {
    let notes = note("c d bb a f g f g").bpm(120).looped().play();
    std::thread::sleep(std::time::Duration::from_secs(10));

    color_eyre::install()?;
    ratatui::run(app)?;

    drop(notes);

    Ok(())
}

fn note_frequency(key: &str) -> Option<f32> {
    match key {
        "c" => Some(261.63),
        "c#" | "db" => Some(277.18),
        "d" => Some(293.66),
        "d#" | "eb" => Some(311.13),
        "e" => Some(329.63),
        "f" => Some(349.23),
        "f#" | "gb" => Some(369.99),
        "g" => Some(392.00),
        "g#" | "ab" => Some(415.30),
        "a" => Some(440.00),
        "a#" | "bb" => Some(466.16),
        "b" => Some(493.88),
        _ => None,
    }
}

pub fn note(input: &str) -> NoteSequence {
    let notes = input
        .split_whitespace()
        .filter_map(note_frequency)
        .collect();

    NoteSequence {
        notes,
        bpm: 120.0,
        looped: false,
    }
}
