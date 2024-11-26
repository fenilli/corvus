use std::time::{Duration, Instant};

pub struct TickState {
    tick_duration: Duration,
    last_update: Instant,
    accumulated_time: Duration,
}

impl TickState {
    pub fn new(ticks_per_second: u32) -> Self {
        Self {
            tick_duration: Duration::from_secs_f64(1.0 / ticks_per_second as f64),
            last_update: Instant::now(),
            accumulated_time: Duration::ZERO,
        }
    }

    pub fn update(&mut self) -> u32 {
        let now = Instant::now();
        let frame_time = now - self.last_update;
        self.last_update = now;
        self.accumulated_time += frame_time;

        let mut ticks: u32 = 0;
        while self.accumulated_time >= self.tick_duration {
            self.accumulated_time -= self.tick_duration;
            ticks += 1;
        }

        ticks
    }

    pub fn tick_duration(&self) -> Duration {
        self.tick_duration
    }
}
