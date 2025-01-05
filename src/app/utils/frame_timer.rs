use std::time::{Duration, Instant};

pub struct FrameTimer {
    fixed_time_step: Duration,
    last_update_time: Instant,
    accumulated_time: Duration,
}

impl FrameTimer {
    pub fn new(fps: u32) -> Self {
        let fixed_time_step = Duration::from_secs_f32(1.0 / fps as f32);

        Self {
            fixed_time_step,
            last_update_time: Instant::now(),
            accumulated_time: Duration::ZERO,
        }
    }

    pub fn advance(&mut self) -> (Vec<f32>, f32) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update_time);
        self.last_update_time = now;

        self.accumulated_time += elapsed;

        let mut fixed_steps = Vec::new();
        while self.accumulated_time >= self.fixed_time_step {
            self.accumulated_time -= self.fixed_time_step;
            fixed_steps.push(self.fixed_time_step.as_secs_f32());
        }

        (fixed_steps, elapsed.as_secs_f32())
    }

    pub fn interpolation_alpha(&self) -> f32 {
        self.accumulated_time.as_secs_f32() / self.fixed_time_step.as_secs_f32()
    }
}
