use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ratatui::{DefaultTerminal, Frame};
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};

#[derive(Clone, Copy)]
enum WaveType {
    Sine,
    Square,
    Triangle,
    Saw,
}

struct State {
    wave: WaveType,
    frequency: f32,
    amplitude: f32,
    phase: f32,
    note_on: bool,
    active_key: Option<char>,
    last_press: Option<Instant>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
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
    let state = Arc::new(Mutex::new(State {
        wave: WaveType::Sine,
        frequency: 261.63,
        amplitude: 0.7,
        phase: 0.0,
        note_on: false,
        active_key: None,
        last_press: None,
    }));
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(0)])
        .split(size);

    let s = state.lock().unwrap();

    let wave_name = match s.wave {
        WaveType::Sine => "Sine",
        WaveType::Square => "Square",
        WaveType::Triangle => "Triangle",
        WaveType::Saw => "Saw",
    };

    let note_info = if s.note_on {
        format!("Playing: {:.2} Hz", s.frequency)
    } else {
        "Playing: ---".to_string()
    };

    let text = format!(
        "TUI Synth (C4–C5)\n\n{}\nWave: {}\nAmplitude: {:.2}\n\n\
                z s x d c v g b h n j m ,  = C4..C5\n\
                1-4 change wave\nEsc exit",
        note_info, wave_name, s.amplitude
    );

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Rust Synth").borders(Borders::ALL));

    frame.render_widget(paragraph, chunks[0]);
}

fn note_frequency(key: char) -> Option<f32> {
    match key {
        'z' => Some(261.63),
        's' => Some(277.18),
        'x' => Some(293.66),
        'd' => Some(311.13),
        'c' => Some(329.63),
        'v' => Some(349.23),
        'g' => Some(369.99),
        'b' => Some(392.00),
        'h' => Some(415.30),
        'n' => Some(440.00),
        'j' => Some(466.16),
        'm' => Some(493.88),
        ',' => Some(523.25),
        _ => None,
    }
}

fn start_audio(state: Arc<Mutex<State>>) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device");
    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate() as f32;

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _| {
            let mut s = state.lock().unwrap();

            for sample in data.iter_mut() {
                if s.note_on {
                    let value = match s.wave {
                        WaveType::Sine => (2.0 * PI * s.phase).sin(),
                        WaveType::Square => {
                            if (2.0 * PI * s.phase).sin() >= 0.0 {
                                1.0
                            } else {
                                -1.0
                            }
                        }
                        WaveType::Triangle => (2.0 * PI * s.phase).sin().asin() * (2.0 / PI),
                        WaveType::Saw => {
                            let mut sum = 0.0;
                            for n in 1..20 {
                                sum += (2.0 * PI * s.phase * n as f32).sin() / n as f32;
                            }
                            sum
                        }
                    };

                    *sample = value * s.amplitude;

                    s.phase += s.frequency / sample_rate;
                    if s.phase >= 1.0 {
                        s.phase -= 1.0;
                    }
                } else {
                    *sample = 0.0;
                }
            }
        },
        |_err| {},
        None,
    )?;

    stream.play()?;
    Ok(stream)
}
