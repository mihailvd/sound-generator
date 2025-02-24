mod wave_types;

use rodio::{source::Source, OutputStream, Sink};
use std::io;
use std::str::FromStr;
use wave_types::*;

const AMPLITUDE: f32 = 0.2;
const SAMPLE_RATE: u32 = 44100;
const DURATION_SECONDS: u64 = 1;

const PROMPT_FREQUENCY: &str = "Enter frequency in Hz (or 'exit' to quit): ";
const PROMPT_WAVE_TYPE: &str = "Enter wave type (sine, square, sawtooth, triangle): ";

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    loop {
        match process_frequency_input() {
            Some(user_input) => {
                if user_input.frequency == 0.0 {
                    continue;
                }

                play_wave(&stream_handle, user_input);
            }
            None => break,
        }
    }
}

fn process_frequency_input() -> Option<WaveInput> {
    println!("{}", PROMPT_FREQUENCY);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed_input = input.trim();
    if trimmed_input.eq_ignore_ascii_case("exit") {
        return None;
    }

    let frequency = match trimmed_input.parse::<f32>() {
        Ok(freq) => freq,
        Err(_) => {
            println!("Invalid frequency. Please enter a valid number.");
            return Some(WaveInput {
                frequency: 0.0,
                wave_type: WaveType::Sine, // Fallback
            });
        }
    };

    println!("{}", PROMPT_WAVE_TYPE);
    let mut wave_type_input = String::new();
    io::stdin().read_line(&mut wave_type_input).unwrap();

    let wave_type = WaveType::from_str(wave_type_input.trim()).unwrap_or_else(|_| {
        println!("Invalid wave type. Defaulting to Sine.");
        WaveType::Sine // Fallback
    });

    Some(WaveInput {
        frequency,
        wave_type,
    })
}

fn play_wave(stream_handle: &rodio::OutputStreamHandle, input: WaveInput) {
    println!("Playing {} Hz...", input.frequency);
    let source = Wave::new(
        input.wave_type,
        input.frequency,
        SAMPLE_RATE,
        DURATION_SECONDS,
    )
    .amplify(AMPLITUDE);

    let sink = Sink::try_new(stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();
}

struct WaveInput {
    frequency: f32,
    wave_type: WaveType,
}
