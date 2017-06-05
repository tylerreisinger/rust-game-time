use float_duration::FloatDuration;
use clock::GameTime;
use super::FrameRateSampler;

pub const DEFAULT_SLOW_THRESHOLD: f64 = 0.95;

pub trait FrameCount {
    fn target_frame_rate(&self) -> f64;
    fn target_time_per_frame(&self) -> FloatDuration;
    fn remaining_frame_time(&self) -> FloatDuration;
}

#[derive(Debug, Clone)]
pub struct FrameCounter<S: FrameRateSampler> {
    target_frame_rate: f64,
    slow_threshold: f64,
    sampler: S,
}

impl<S: FrameRateSampler> FrameCounter<S> {
    pub fn new(target_frame_rate: f64, sampler: S) -> FrameCounter<S> {
        FrameCounter {
            target_frame_rate,
            slow_threshold: DEFAULT_SLOW_THRESHOLD,
            sampler,
        }
    }
    pub fn set_slow_threshold(&mut self, val: f64) -> &mut FrameCounter<S> {
        self.slow_threshold = val;
        self
    }
    pub fn slow_threshold(&self) -> f64 {
        self.slow_threshold
    }

    pub fn set_target_frame_rate(&mut self, val: f64) -> &mut FrameCounter<S> {
        self.target_frame_rate = val;
        self
    }
    pub fn target_frame_rate(&self) -> f64 {
        self.target_frame_rate
    }
    pub fn target_time_per_frame(&self) -> FloatDuration {
        FloatDuration::seconds(1.0) / self.target_frame_rate
    }

    pub fn remaining_frame_time(&self, time: &GameTime) -> FloatDuration {
        self.target_time_per_frame() - time.elapsed_time_since_frame_start()
    }
    pub fn average_frame_rate(&self) -> f64 {
        self.sampler.average_frame_rate()
    }
    pub fn last_frame_rate(&self, time: &GameTime) -> f64 {
        1.0 / time.elapsed_game_time().as_seconds()
    }
    pub fn is_running_slow(&self, time: &GameTime) -> bool {
        let ratio = self.target_time_per_frame().as_seconds() /
                    time.elapsed_wall_time().as_seconds();
        ratio <= self.slow_threshold
    }
    pub fn is_saturated(&self) -> bool {
        self.sampler.is_saturated()
    }
    pub fn sampler(&self) -> &S {
        &self.sampler
    }

    pub fn tick(&mut self, time: &GameTime) {
        self.sampler.tick(time);
    }
}
