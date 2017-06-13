//! Utilities for frame rate computation and management.
pub mod counter;
pub mod sample;

pub use self::counter::{FrameCounter, FrameCount};
pub use self::sample::{FrameRateSampler, RunningAverageSampler};
