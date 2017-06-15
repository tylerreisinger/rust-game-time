//! Provides utilities for tracking frame rate.
use std::fmt::Debug;

use float_duration::FloatDuration;
use clock::GameTime;
use super::FrameRateSampler;

/// The default slow threshold for `FrameCounter`.
pub const DEFAULT_SLOW_THRESHOLD: f64 = 0.95;

/// A frame rate counter and tracker.
pub trait FrameCount: Debug {
    /// The target frame rate for the simulation.
    fn target_frame_rate(&self) -> f64;
    /// The target wall time for each frame.
    fn target_time_per_frame(&self) -> FloatDuration;
    /// The duration remaining to reach the optimal frame time.
    ///
    /// The duration can be negative if the current frame has
    /// executed longer than the optimal time.
    fn remaining_frame_time(&self, time: &GameTime) -> FloatDuration;
    /// Update the counter for a new frame.
    fn tick(&mut self, time: &GameTime);
    /// The average frame rate for the current frame.
    fn average_frame_rate(&self) -> f64;
    /// Return whether the simulation is running slowly.
    fn is_running_slow(&self, time: &GameTime) -> bool;
}

/// A basic frame rate counter.
#[derive(Debug, Clone)]
pub struct FrameCounter<S: FrameRateSampler> {
    target_frame_rate: f64,
    slow_threshold: f64,
    sampler: S,
}

impl<S: FrameRateSampler> FrameCounter<S> {
    /// Create a new `FrameCounter` with a target frame rate and sampler.
    pub fn new(target_frame_rate: f64, sampler: S) -> FrameCounter<S> {
        FrameCounter {
            target_frame_rate,
            slow_threshold: DEFAULT_SLOW_THRESHOLD,
            sampler,
        }
    }
    /// Set a new slow threshold.
    pub fn set_slow_threshold(&mut self, val: f64) -> &mut FrameCounter<S> {
        self.slow_threshold = val;
        self
    }
    /// Return the current slow threshold.
    ///
    /// If the current frame rate divided by the target frame rate
    /// is less than this value, then
    /// [`FrameCount::is_running_slowly`](./trait.FrameCount.html#tymethod.is_running_slowly)
    /// will return true.
    pub fn slow_threshold(&self) -> f64 {
        self.slow_threshold
    }

    /// Set the target frame rate.
    pub fn set_target_frame_rate(&mut self, val: f64) -> &mut FrameCounter<S> {
        self.target_frame_rate = val;
        self
    }
    /// Return true if the sampler is saturated.
    pub fn is_saturated(&self) -> bool {
        self.sampler.is_saturated()
    }
    /// Return a reference to the sampler object.
    pub fn sampler(&self) -> &S {
        &self.sampler
    }
}

impl<S: FrameRateSampler> FrameCount for FrameCounter<S> {
    fn target_frame_rate(&self) -> f64 {
        self.target_frame_rate
    }
    fn target_time_per_frame(&self) -> FloatDuration {
        FloatDuration::seconds(1.0) / self.target_frame_rate
    }
    fn remaining_frame_time(&self, time: &GameTime) -> FloatDuration {
        self.target_time_per_frame() - time.elapsed_time_since_frame_start()
    }
    fn tick(&mut self, time: &GameTime) {
        self.sampler.tick(time);
    }
    fn average_frame_rate(&self) -> f64 {
        self.sampler.average_frame_rate()
    }
    fn is_running_slow(&self, time: &GameTime) -> bool {
        let ratio = self.target_time_per_frame().as_seconds() /
            time.elapsed_wall_time().as_seconds();
        ratio <= self.slow_threshold
    }
}
