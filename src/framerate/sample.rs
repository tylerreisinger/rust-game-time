use std::fmt::Debug;

use std::collections::VecDeque;

use clock::GameTime;

pub const DEFAULT_NUM_SAMPLES: u32 = 128;

pub trait FrameRateSampler: Debug {
    fn tick(&mut self, time: &GameTime);
    fn average_frame_rate(&self) -> f64;
    fn is_saturated(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct RunningAverageSampler {
    max_samples: u32,
    current_samples: u32,
    current_average: f64,
}

#[derive(Debug, Clone)]
pub struct LinearAverageSampler {
    past_data: VecDeque<f64>,
}

impl RunningAverageSampler {
    pub fn new() -> RunningAverageSampler {
        RunningAverageSampler::with_max_samples(DEFAULT_NUM_SAMPLES)
    }

    pub fn with_max_samples(max_samples: u32) -> RunningAverageSampler {
        RunningAverageSampler {
            max_samples,
            current_samples: 0,
            current_average: 0.0,
        }
    }
}

impl FrameRateSampler for RunningAverageSampler {
    fn tick(&mut self, time: &GameTime) {
        if !self.is_saturated() {
            self.current_samples += 1;
        }
        let num_samples = self.current_samples;

        let effective_fps = 1.0 / time.elapsed_game_time().as_seconds();
        let new_average = ((self.current_average * (num_samples - 1) as f64) + effective_fps) /
                          (num_samples as f64);

        self.current_average = new_average;
    }
    fn average_frame_rate(&self) -> f64 {
        self.current_average
    }
    fn is_saturated(&self) -> bool {
        self.current_samples == self.max_samples
    }
}

impl LinearAverageSampler {
    pub fn new() -> LinearAverageSampler {
        LinearAverageSampler::with_max_samples(DEFAULT_NUM_SAMPLES)
    }
    pub fn with_max_samples(max_samples: u32) -> LinearAverageSampler {
        LinearAverageSampler { past_data: VecDeque::with_capacity(max_samples as usize) }
    }
}

impl FrameRateSampler for LinearAverageSampler {
    fn tick(&mut self, time: &GameTime) {
        let effective_fps = 1.0 / time.elapsed_game_time().as_seconds();

        if self.is_saturated() {
            self.past_data.pop_front();
        }
        self.past_data.push_back(effective_fps);
    }

    fn average_frame_rate(&self) -> f64 {
        let sum: f64 = self.past_data.iter().sum();
        sum / (self.past_data.len() as f64)
    }
    fn is_saturated(&self) -> bool {
        self.past_data.len() == self.past_data.capacity()
    }
}
