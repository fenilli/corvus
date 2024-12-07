use std::time::{Duration, Instant};

pub struct Timestep {
    fixed_delta_time: Duration,
    last_frame_time: Instant,
    accumulator: Duration,
}

impl Timestep {
    pub fn new(fps: u32) -> Self {
        let fixed_delta_time = Duration::from_secs_f32(1.0 / fps as f32);

        Self {
            fixed_delta_time,
            last_frame_time: Instant::now(),
            accumulator: Duration::ZERO,
        }
    }

    pub fn update(&mut self) -> (Vec<f32>, f32) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;

        self.accumulator += frame_time;

        let mut fixed_deltas: Vec<f32> = Vec::new();
        while self.accumulator >= self.fixed_delta_time {
            self.accumulator -= self.fixed_delta_time;
            fixed_deltas.push(self.fixed_delta_time.as_secs_f32());
        }

        let variable_delta = frame_time.as_secs_f32();
        (fixed_deltas, variable_delta)
    }
}
