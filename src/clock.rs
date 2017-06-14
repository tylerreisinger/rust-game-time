//! Types for counting and recording time in a simulation.
//!
//! `clock` is the primary module for time tracking in `game_time`. It
//! provides two primary types: `GameClock`, a "clock" for tracking frames
//! and time progression within the simulation and `GameTime`, a specific
//! point in time within the simulation.
use std::thread;
use std::time;

use chrono;
use float_duration::{FloatDuration, TimePoint};
use progression::TimeProgression;

use framerate::FrameCount;

/// A specific point of time in a simulation.
///
/// `GameTime` knows both the wall time and game time of the simulation at a
/// fixed time. A `GameTime` object is usually created by calling
/// [`tick`](./struct.GameClock.html#method.tick) on a [`GameClock`](./struct.GameClock.html)
/// object.
#[derive(Debug, Clone)]
pub struct GameTime {
    frame_wall_time: chrono::DateTime<chrono::Local>,
    frame_game_time: time::Duration,
    elapsed_game_time: FloatDuration,
    elapsed_wall_time: FloatDuration,
    frame_number: u64,
}

/// Time-tracking for use in real-time simulations.
///
/// `GameClock` provides time tracking for simulations. It
/// tracks total time the simulation has run for as well as
/// the elapsed time between individual frames.
///
/// In addition to tracking wall time, it tracks "game time"
/// which is the time used by the simulation itself. This game time
/// can be coupled directly to wall time, or can be changed by a fixed
/// time step per frame.
///
/// `GameClock` uses [`GameTime`](./struct.GameTime.html) objects
/// to report the time at each frame to the program. The [`tick`](./fn.tick.html)
/// tells `GameClock` when a new frame is started, and returns
/// the `GameTime` object for that frame. This object can then be passed
/// to the rest of the simulation independently of `GameClock`.
#[derive(Debug, Clone)]
pub struct GameClock {
    last_frame_time: GameTime,
    start_wall_time: chrono::DateTime<chrono::Local>,
    total_game_time: time::Duration,
    current_frame: u64,
    clock_multiplier: f64,
}

impl GameClock {
    /// Construct a new `GameClock` object, initialized to start at
    /// zero game time and a wall time of `chrono::Local::now()`.
    pub fn new() -> GameClock {
        let now = chrono::Local::now();
        let start_game_time = GameTime {
            frame_wall_time: now,
            frame_game_time: time::Duration::new(0, 0),
            elapsed_game_time: FloatDuration::zero(),
            elapsed_wall_time: FloatDuration::zero(),
            frame_number: 0,
        };

        GameClock {
            last_frame_time: start_game_time,
            start_wall_time: now,
            total_game_time: time::Duration::new(0, 0),
            current_frame: 0,
            clock_multiplier: 1.0,
        }
    }

    /// Return the current frame number.
    ///
    /// The frame number starts at `0` for "before the first frame"
    /// and increases by 1 every time `tick` is called.
    pub fn current_frame_number(&self) -> u64 {
        self.current_frame
    }
    /// Return the wall time when the `GameClock` was created.
    pub fn start_wall_time(&self) -> chrono::DateTime<chrono::Local> {
        self.start_wall_time
    }
    /// Return the wall time at the start of the current frame.
    ///
    /// This is equivalent to the value returned by
    /// `last_frame_time().frame_wall_time()`
    pub fn frame_wall_time(&self) -> chrono::DateTime<chrono::Local> {
        self.last_frame_time.frame_wall_time
    }

    /// Return the amount of wall time elapsed since the start of the current frame.
    pub fn frame_elapsed_time(&self) -> FloatDuration {
        chrono::Local::now()
            .float_duration_since(self.frame_wall_time())
            .unwrap()
    }
    /// Return the [`GameTime`](./struct.GameTime.html) for the current frame.
    pub fn last_frame_time(&self) -> &GameTime {
        &self.last_frame_time
    }
    /// Return the rate at which game time is increasing.
    pub fn clock_multiplier(&self) -> f64 {
        self.clock_multiplier
    }
    /// Set the rate at which game time is increasing.
    pub fn with_clock_multiplier(&mut self, val: f64) -> &mut GameClock {
        self.clock_multiplier = val;
        self
    }

    /// Mark the start of a new frame, updating time statistics.
    ///
    /// The `GameTime` for the new frame is returned. This gives the time
    /// statistics for the entirety of the current frame. It is cached and
    /// can be later obtained by calling `last_frame_time`.
    pub fn tick<T>(&mut self, time_progress: &mut T) -> GameTime
        where T: TimeProgression + ?Sized
    {
        let frame_start = chrono::Local::now();

        self.current_frame += 1;

        let elapsed_wall_time = frame_start
            .float_duration_since(self.frame_wall_time())
            .unwrap();

        let elapsed_game_time = time_progress.compute_game_time(&elapsed_wall_time) *
                                self.clock_multiplier;
        let total_game_time = self.total_game_time + elapsed_game_time.to_std().unwrap();

        self.total_game_time = total_game_time;

        let time = GameTime {
            frame_wall_time: frame_start,
            frame_game_time: total_game_time,
            elapsed_game_time,
            elapsed_wall_time,
            frame_number: self.current_frame,
        };

        self.last_frame_time = time.clone();

        time
    }

    /// Put the current thread to sleep if necessary in order to maintain the target frame rate.
    ///
    /// If the current frame has taken more time than the target frame rate allows, then the
    /// thread will not sleep. Otherwise it will sleep for
    /// `counter.target_time_per_frame() - self.frame_elapsed_time()`
    ///
    /// This method relies on the passed function `f` to actually perform the sleep.
    /// `f` receives the amount of sleep time requested and it is up to itself to
    /// sleep for that amount of time. If you don't care how the sleep is performed,
    /// use the [`sleep_remaining`](./struct.GameClock.html#method.sleep_remaining)
    /// method instead.
    pub fn sleep_remaining_via<C, F>(&mut self, counter: &C, f: F)
        where C: FrameCount + ?Sized,
              F: FnOnce(FloatDuration)
    {
        let remaining_time = counter.target_time_per_frame() -
                             self.last_frame_time.elapsed_time_since_frame_start();
        if !remaining_time.is_negative() {
            f(remaining_time)
        }
    }

    /// Put the current thread to sleep if necessary in order to maintain the target frame rate.
    ///
    /// If the current frame has taken more time than the target frame rate allows, then the
    /// thread will not sleep. Otherwise it will sleep for
    /// `counter.target_time_per_frame() - self.frame_elapsed_time()`
    ///
    /// This method uses [`std::thread::sleep`](https://doc.rust-lang.org/std/thread/fn.sleep.html)
    /// to put the thread to sleep. If a different sleep function is desired, use
    /// the [`sleep_remaining_via`](./struct.GameClock.html#method.sleep_remaining_via)
    /// method instead.
    pub fn sleep_remaining<C>(&mut self, counter: &C)
        where C: FrameCount + ?Sized
    {
        self.sleep_remaining_via(counter, |rem| thread::sleep(rem.to_std().unwrap()))
    }
}

impl Default for GameClock {
    fn default() -> GameClock {
        GameClock::new()
    }
}

impl GameTime {
    /// Construct a `GameTime` object directly.
    ///
    /// This is useful primarily for writing tests or for constructing a
    /// `GameTime` without using a `GameClock`.
    pub fn new(frame_wall_time: chrono::DateTime<chrono::Local>,
               frame_game_time: time::Duration,
               elapsed_game_time: FloatDuration,
               elapsed_wall_time: FloatDuration,
               frame_number: u64)
               -> GameTime {
        GameTime {
            frame_wall_time,
            frame_game_time,
            elapsed_game_time,
            elapsed_wall_time,
            frame_number,
        }
    }

    /// The game time at the time of creation of this `GameTime` object.
    pub fn frame_game_time(&self) -> time::Duration {
        self.frame_game_time
    }
    /// The wall time at the time of creation of this `GameTime` object.
    pub fn frame_wall_time(&self) -> chrono::DateTime<chrono::Local> {
        self.frame_wall_time
    }
    /// The amount of game time that passed since the previous frame.
    pub fn elapsed_game_time(&self) -> FloatDuration {
        self.elapsed_game_time
    }
    /// The amount of wall time that passed since the previous frame.
    pub fn elapsed_wall_time(&self) -> FloatDuration {
        self.elapsed_wall_time
    }
    /// The amount of elapsed wall time since the start of the current frame.
    ///
    /// This value is computed from the current instant when called, based
    /// on the frame start time. This can be used for intra-frame profiling.
    pub fn elapsed_time_since_frame_start(&self) -> FloatDuration {
        chrono::Local::now()
            .float_duration_since(self.frame_wall_time)
            .unwrap()
    }
    /// The index of the current frame.
    ///
    /// This value increases by 1 for each frame created, and represents the total
    /// number of frames executed in the simulation.
    pub fn frame_number(&self) -> u64 {
        self.frame_number
    }
    /// Return the instantaneous frame rate between the last and current frames.
    ///
    /// The "instantaneous" frame rate is computed from the last frame's elapsed
    /// time, and takes no previous frames into account.
    ///
    /// For a more stable frame rate, use a [`FrameCount`](../framerate/counter/trait.FrameCount.html)
    /// object.
    pub fn instantaneous_frame_rate(&self) -> f64 {
        1.0 / self.elapsed_game_time.as_seconds()
    }
}
