use std::time::{Duration, Instant};

pub struct Clock {
    frame_duration: Duration,
    accumulated_time: Duration,
    last_update: Instant,
}

impl Clock {
    pub fn new(fps: u32) -> Self {
        Self {
            frame_duration: Duration::from_secs_f64(1.0 / fps as f64),
            accumulated_time: Duration::ZERO,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) -> ClockIterator {
        let now = Instant::now();
        let frame_time = now - self.last_update;
        self.last_update = now;
        self.accumulated_time += frame_time;

        ClockIterator { clock: self }
    }
}

pub struct ClockIterator<'a> {
    clock: &'a mut Clock,
}

impl<'a> Iterator for ClockIterator<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.clock.accumulated_time >= self.clock.frame_duration {
            self.clock.accumulated_time -= self.clock.frame_duration;
            return Some(self.clock.frame_duration.as_secs_f32());
        }

        None
    }
}
