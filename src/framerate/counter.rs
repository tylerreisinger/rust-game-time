use duration::FloatDuration;
use clock::GameTime;
use super::FrameRateSampler;

#[derive(Debug, Clone)]
pub struct FrameCounter<S: FrameRateSampler> {
    target_frame_rate: f64,
    sampler: S,
}

impl<S: FrameRateSampler> FrameCounter<S> {
    pub fn new(target_frame_rate: f64, sampler: S) -> FrameCounter<S> {
        FrameCounter { target_frame_rate , sampler }
    }

    pub fn target_frame_rate(&self) -> f64 {
        self.target_frame_rate
    }
    pub fn target_time_per_frame(&self) -> FloatDuration {
        FloatDuration::seconds(1.0) / self.target_frame_rate
    }
    pub fn average_frame_rate(&self) -> f64 {
        self.sampler.average_frame_rate()
    }
    pub fn last_frame_rate(&self, time: &GameTime) -> f64 {
        1.0 / time.elapsed_game_time().as_seconds()
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
