//! Types for computing game time from wall time.
use float_duration::FloatDuration;
use framerate::FrameCount;

/// Compute elapsed game time for a frame.
pub trait TimeStep {
    /// Compute the time step for the next frame.
    fn time_step(&self, wall_time: &FloatDuration) -> FloatDuration;
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
where
    C: 'a + FrameCount + ?Sized,
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
    /// Construct a new `ConstantStep` that does not advance game time.
    pub fn null_step() -> ConstantStep {
        ConstantStep::new(FloatDuration::zero())
    }
}

impl TimeStep for VariableStep {
    fn time_step(&self, wall_time: &FloatDuration) -> FloatDuration {
        *wall_time
    }
}
impl<'a, C> TimeStep for FixedStep<'a, C>
where
    C: 'a + FrameCount + ?Sized,
{
    fn time_step(&self, _: &FloatDuration) -> FloatDuration {
        self.counter.target_time_per_frame()
    }
}
impl TimeStep for ConstantStep {
    fn time_step(&self, _: &FloatDuration) -> FloatDuration {
        self.step
    }
}
