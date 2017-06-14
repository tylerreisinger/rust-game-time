//! Types for computing game time from wall time.
use float_duration::FloatDuration;
use framerate::FrameCount;

/// Compute elapsed game time for a frame.
pub trait TimeProgression {
    fn compute_game_time(&mut self, wall_time: &FloatDuration) -> FloatDuration;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VariableStep {}
#[derive(Debug)]
pub struct FixedStep<'a, C: 'a + FrameCount + ?Sized> {
    counter: &'a mut C,
}

impl VariableStep {
    pub fn new() -> VariableStep {
        VariableStep {}
    }
}
impl<'a, C> FixedStep<'a, C>
    where C: 'a + FrameCount + ?Sized
{
    pub fn new(counter: &'a mut C) -> FixedStep<'a, C> {
        FixedStep { counter: counter }
    }
}

impl TimeProgression for VariableStep {
    fn compute_game_time(&mut self, wall_time: &FloatDuration) -> FloatDuration {
        *wall_time
    }
}
impl<'a, C> TimeProgression for FixedStep<'a, C>
    where C: 'a + FrameCount + ?Sized
{
    fn compute_game_time(&mut self, _: &FloatDuration) -> FloatDuration {
        self.counter.target_time_per_frame()
    }
}
