use std::time::{Duration, Instant};

pub struct Timestep {
    fixed_delta_time: Duration,
    last_frame_time: Instant,
    accumulator: Duration,

    fixed_deltas: Vec<Duration>,
    variable_delta: Duration,
}

impl Timestep {
    pub fn new(fps: u32) -> Self {
        let fixed_delta_time = Duration::from_secs_f32(1.0 / fps as f32);

        Self {
            fixed_delta_time,
            last_frame_time: Instant::now(),
            accumulator: Duration::ZERO,

            fixed_deltas: Vec::new(),
            variable_delta: Duration::ZERO,
        }
    }

    pub fn delta_time(&self) -> f32 {
        self.variable_delta.as_secs_f32()
    }

    pub fn fixed_deltas_time(&self) -> Vec<f32> {
        self.fixed_deltas
            .iter()
            .map(|time| time.as_secs_f32())
            .collect()
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;

        self.accumulator += frame_time;

        let mut fixed_deltas: Vec<Duration> = Vec::new();
        while self.accumulator >= self.fixed_delta_time {
            self.accumulator -= self.fixed_delta_time;
            fixed_deltas.push(self.fixed_delta_time);
        }

        self.fixed_deltas = fixed_deltas;
        self.variable_delta = frame_time;
    }

    pub fn reset(&mut self) {
        self.fixed_deltas = Vec::new();
        self.variable_delta = Duration::ZERO;
    }
}
