//! Utilities for computing average frame rate.
use std::fmt::Debug;

use std::collections::VecDeque;

use clock::GameTime;

/// The default number of samples for frame rate samplers.
pub const DEFAULT_NUM_SAMPLES: u32 = 64;

/// Frame rate computation.
///
/// `FrameRateSampler` provides methods to take the time at each frame and compute
/// a frame rate metric through some method.
pub trait FrameRateSampler: Debug {
    /// Update the frame rate with a new frame.
    fn tick(&mut self, time: &GameTime);
    /// Return the current frame rate measure.
    fn average_frame_rate(&self) -> f64;
    /// Return true if the number of samples fills the cache.
    fn is_saturated(&self) -> bool;
}

/// A frame rate sampler that computes a moving average from past frames without caching data.
///
/// `RunningAverageSampler` computes the average value by computing `(avg*(N-1) + next) / N`.
/// This method does not require caching past frames, but is sensitive to large outliers
/// influencing the value for many frames.
#[derive(Debug, Clone, Default)]
pub struct RunningAverageSampler {
    max_samples: u32,
    current_samples: u32,
    current_average: f64,
}

/// A frame rate sampler that computes the average frame rate of a number of past frames.
#[derive(Debug, Clone, Default)]
pub struct LinearAverageSampler {
    past_data: VecDeque<f64>,
}

impl RunningAverageSampler {
    /// Construct a new `RunningAverageSampler` with a default sample size.
    pub fn new() -> RunningAverageSampler {
        RunningAverageSampler::with_max_samples(DEFAULT_NUM_SAMPLES)
    }

    /// Construct a `RunningAverageSampler` with a specified sample size.
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
    /// Construct a new `LinearAverageSampler` with a default sample size.
    pub fn new() -> LinearAverageSampler {
        LinearAverageSampler::with_max_samples(DEFAULT_NUM_SAMPLES)
    }
    /// Construct a new LinearAverageSampler` with a specified sample size.
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
