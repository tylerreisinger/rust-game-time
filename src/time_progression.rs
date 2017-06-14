//! Types for computing game time from wall time.
use float_duration::FloatDuration;
use framerate::FrameCount;

/// Compute elapsed game time for a frame.
pub trait TimeProgression {
    /// Compute the game time.
    fn compute_game_time(&mut self, wall_time: &FloatDuration) -> FloatDuration;
}

/// A time step based on elapsed wall time.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VariableStep {}
/// A fixed time step derived from a set frame rate.
#[derive(Debug)]
pub struct FixedStep<'a, C: 'a + FrameCount + ?Sized> {
    counter: &'a mut C,
}
/// A specific, constant time step.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ConstantStep {
    step: FloatDuration,
}

impl VariableStep {
    /// Construct a new `VariableStep` object.
    pub fn new() -> VariableStep {
        VariableStep {}
    }
}
impl<'a, C> FixedStep<'a, C>
    where C: 'a + FrameCount + ?Sized
{
    /// Construct a new `FixedStep` object based on the set frame rate in `counter`.
    pub fn new(counter: &'a mut C) -> FixedStep<'a, C> {
        FixedStep { counter: counter }
    }
}
impl ConstantStep {
    /// Construct a new `ConstantStep` object with a set time step `step`.
    pub fn new(step: FloatDuration) -> ConstantStep {
        ConstantStep { step }
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
impl TimeProgression for ConstantStep {
    fn compute_game_time(&mut self, _: &FloatDuration) -> FloatDuration {
        self.step.clone()
    }
}
