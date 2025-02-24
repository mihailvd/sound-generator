use rodio::Source;
use std::f32::consts::PI;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug)]
pub enum WaveType {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

impl WaveType {
    fn generate_phase(&self, normalized_clock: f32, frequency: f32) -> f32 {
        match self {
            WaveType::Sine => (2.0 * PI * frequency * normalized_clock).sin(),
            WaveType::Square => {
                if (2.0 * PI * frequency * normalized_clock).sin() >= 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
            WaveType::Sawtooth => {
                2.0 * (frequency * normalized_clock - (frequency * normalized_clock).floor()) - 1.0
            }
            WaveType::Triangle => (2.0
                * ((2.0 * frequency * normalized_clock).floor()
                    - (frequency * normalized_clock).floor())
                .abs()
                - 1.0)
                .abs(),
        }
    }
}

impl FromStr for WaveType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "sine" => Ok(WaveType::Sine),
            "square" => Ok(WaveType::Square),
            "sawtooth" => Ok(WaveType::Sawtooth),
            "triangle" => Ok(WaveType::Triangle),
            _ => Err(format!("Unknown wave type: {}", input)),
        }
    }
}

pub struct Wave {
    duration: u64,
    frequency: f32,
    sample_rate: u32,
    sample_clock: f32,
    wave_type: WaveType,
}

impl Wave {
    pub fn new(wave_type: WaveType, frequency: f32, sample_rate: u32, duration: u64) -> Self {
        Self {
            wave_type,
            duration,
            frequency,
            sample_rate,
            sample_clock: 0.0,
        }
    }
}

impl Iterator for Wave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sample_clock >= self.duration as f32 {
            return None;
        }

        let phase = self
            .wave_type
            .generate_phase(self.sample_clock, self.frequency);

        self.sample_clock += 1.0 / self.sample_rate as f32;

        Some(phase)
    }
}

impl Source for Wave {
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
