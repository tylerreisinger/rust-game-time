use super::duration::FloatDuration;
use super::clock::GameTime;

pub const DEFAULT_NUM_SAMPLES: u32 = 128;

pub trait FrameRateSampler {
    fn tick(&mut self, time: &GameTime) -> f64;
    fn average_frame_rate(&self) -> f64;
}

pub struct MovingAverageSampler {
    max_samples: u32,
    current_samples: u32,
    current_average: f64,
}

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
    pub fn sampler(&self) -> &S {
        &self.sampler
    }

    pub fn tick(&mut self, time: &GameTime) {
        self.sampler.tick(time);        
    }
}

impl MovingAverageSampler {
    pub fn new() -> MovingAverageSampler {
        MovingAverageSampler::with_max_samples(DEFAULT_NUM_SAMPLES)
    }

    pub fn with_max_samples(max_samples: u32) -> MovingAverageSampler {
        MovingAverageSampler {
            max_samples,
            current_samples: 0,
            current_average: 0.0
        }
    }
}

impl FrameRateSampler for MovingAverageSampler {
    fn tick(&mut self, time: &GameTime) -> f64 {
        if self.current_samples != self.max_samples {
            self.current_samples += 1;
        }
        let num_samples = self.current_samples;

        let effective_fps = 1.0 / time.elapsed_game_time().as_seconds();
        let new_average = 
            ((self.current_average * (num_samples-1) as f64) + effective_fps) 
                / (num_samples as f64);

        self.current_average = new_average;
        new_average
    }
    fn average_frame_rate(&self) -> f64 {
        self.current_average
    }
}
