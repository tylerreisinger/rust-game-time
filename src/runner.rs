//! Implements the [`FrameRunner`](./runner/struct.FrameRunner.html) struct for managing frame simulations.
use chrono;

use clock::{GameTime, GameClock};
use framerate::counter::FrameCount;
use step::TimeStep;

/// A helper type for running frame simulations with a frame counter.
///
/// `FrameRunner` combines a [`GameClock`](../clock/struct.GameTime.html)
/// and a [`FrameCount`](../framerate/counter/struct.FrameCounter.html) object,
/// tracking each frame as well as a framerate. `FrameRunner` provides
/// a `tick` method like `GameClock`, and updates both the `GameClock` and
/// `FrameCount` objects contained.
#[derive(Debug)]
pub struct FrameRunner<C: FrameCount, T: TimeStep> {
    clock: GameClock,
    counter: C,
    time_progress: T,
}

impl<C, T> FrameRunner<C, T>
where
    C: FrameCount,
    T: TimeStep,
{
    /// Construct a new `FrameRunner` from a `GameClock` and a `FrameCount`.
    pub fn new(clock: GameClock, counter: C, time_progress: T) -> FrameRunner<C, T> {
        FrameRunner {
            clock,
            counter,
            time_progress,
        }
    }

    /// Get a reference to the contained `GameClock`.
    pub fn clock(&self) -> &GameClock {
        &self.clock
    }
    /// Get a mutable reference to the contained `GameClock`.
    pub fn clock_mut(&mut self) -> &mut GameClock {
        &mut self.clock
    }
    /// Get a reference to the contained `FrameCount`.
    pub fn counter(&self) -> &C {
        &self.counter
    }
    /// Get a mutable reference to the contained `FrameCount`.
    pub fn counter_mut(&mut self) -> &mut C {
        &mut self.counter
    }
    /// Mark the start of a new frame, updating time and frame rate statistics.
    ///
    /// The `GameTime` for the new frame is returned, with the same properties as that
    /// returned from [`GameClock::tick`](../clock/struct.GameClock.html#method.tick).
    pub fn tick(&mut self) -> GameTime {
        let time = self.clock.tick(&mut self.time_progress);
        time
    }

    /// Mark the start of a new frame with a specified wall time, updating time statistics.
    ///
    /// This function is like `tick` but allows for the start time for the
    /// frame to be specified.
    pub fn tick_with_wall_time(
        &mut self,
        frame_start: chrono::DateTime<chrono::Local>,
    ) -> GameTime {
        self.clock.tick_with_wall_time(
            &mut self.time_progress,
            frame_start,
        )
    }

    /// Perform one frame of the simulation using `frame_fn`.
    ///
    /// The closure is passed the `GameTime` for the frame by calling `tick`
    /// and will call
    /// [`GameClock::sleep_remaining`](../clock/struct.GameClock.html#method.sleep_remaining)
    /// after the closure has ended.
    pub fn do_frame<F>(&mut self, frame_fn: F)
    where
        F: FnOnce(GameTime),
    {
        let time = self.tick();
        frame_fn(time);
        self.clock.sleep_remaining(&self.counter);
    }
}
