use rodio::{source::Source, OutputStream, Sink};
use std::f32::consts::PI;
use std::io;
use std::time::Duration;

const AMPLITUDE: f32 = 0.2;
const SAMPLE_RATE: u32 = 44100;
const DURATION_SECONDS: u64 = 1;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    loop {
        match process_frequency_input() {
            Some(frequency) => {
                println!("Playing {} Hz...", frequency);
                let source =
                    SineWave::new(frequency, SAMPLE_RATE, DURATION_SECONDS).amplify(AMPLITUDE);
                let sink = Sink::try_new(&stream_handle).unwrap();
                sink.append(source);
                sink.sleep_until_end();
            }
            None => break,
        }
    }
}

fn process_frequency_input() -> Option<f32> {
    println!("Enter frequency in Hz (or 'exit' to quit): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "exit" {
        return None;
    }

    match input.parse::<f32>() {
        Ok(frequency) => Some(frequency),
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            None
        }
    }
}

struct SineWave {
    duration: u64,
    frequency: f32,
    sample_rate: u32,
    sample_clock: f32,
}

impl SineWave {
    fn new(frequency: f32, sample_rate: u32, duration: u64) -> Self {
        Self {
            duration,
            frequency,
            sample_rate,
            sample_clock: 0.0,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sample_clock >= self.duration as f32 {
            return None;
        }
        let value = (2.0 * PI * self.frequency * self.sample_clock).sin();
        self.sample_clock += 1.0 / self.sample_rate as f32;
        Some(value)
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs(self.duration))
    }
}
