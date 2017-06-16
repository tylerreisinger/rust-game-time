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
use step::TimeStep;

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
/// which is the time used by the simulation itself. This game time is updated
/// each frame and
/// can be coupled directly to wall time [`VariableStep`](../step/struct.VariableStep.html)
/// or can be updated by a fixed amount (see the [`step`](../step/index.html) module for more
/// options).
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

/// A [`GameClock`](./struct.GameClock.html) builder,
/// allowing for customization of the initial time and parameters.
///
/// `GameClockBuilder` offers fine control over the initial state of a `GameClock`. For
/// most cases, using [`GameClock::new()`](./struct.GameClock.html#method.new) is good enough.
/// However, it can be useful to have more control in some situations, especially testing.
#[derive(Debug, Clone)]
pub struct GameClockBuilder {
    start_game_time: time::Duration,
    start_wall_time: chrono::DateTime<chrono::Local>,
    start_frame: u64,
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
    pub fn set_clock_multiplier(&mut self, val: f64) -> &mut GameClock {
        self.clock_multiplier = val;
        self
    }

    /// Mark the start of a new frame, updating time statistics.
    ///
    /// The `GameTime` for the new frame is returned. This gives the time
    /// statistics for the entirety of the current frame. It is cached and
    /// can be later obtained by calling `last_frame_time`.
    ///
    /// `time_progress` is a [`TimeStep`](../step/trait.TimeStep.html) reference used to
    /// compute the elapsed game time for the frame..
    pub fn tick<T>(&mut self, time_progress: &mut T) -> GameTime
    where
        T: TimeStep + ?Sized,
    {
        let frame_start = chrono::Local::now();

        self.current_frame += 1;

        let elapsed_wall_time = frame_start
            .float_duration_since(self.frame_wall_time())
            .unwrap();

        let elapsed_game_time = time_progress.time_step(&elapsed_wall_time) * self.clock_multiplier;
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
    where
        C: FrameCount + ?Sized,
        F: FnOnce(FloatDuration),
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
    where
        C: FrameCount + ?Sized,
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
    /// For a more stable frame rate, use a
    /// [`FrameCount`](../framerate/counter/trait.FrameCount.html) object.
    pub fn instantaneous_frame_rate(&self) -> f64 {
        1.0 / self.elapsed_game_time.as_seconds()
    }
}

impl GameClockBuilder {
    /// Construct a new `GameClockBuilder` with default values.
    ///
    /// Calling `build` on the returned object returns immediately gives the same
    /// result as `GameClock::new()`.
    pub fn new() -> GameClockBuilder {
        GameClockBuilder {
            start_game_time: time::Duration::new(0, 0),
            start_wall_time: chrono::Local::now(),
            start_frame: 0,
            clock_multiplier: 1.0,
        }
    }

    /// Set the initial game time when the game is started.
    ///
    /// Defaults to zero.
    pub fn start_game_time(&mut self, time: time::Duration) -> &mut GameClockBuilder {
        self.start_game_time = time;
        self
    }
    /// Set the initial wall time when the game is started.
    ///
    /// Defaults to `chrono::Local::now()`.
    pub fn start_wall_time(
        &mut self,
        time: chrono::DateTime<chrono::Local>,
    ) -> &mut GameClockBuilder {
        self.start_wall_time = time;
        self
    }
    /// Set the initial frame number.
    ///
    /// Defaults to `0`.
    pub fn start_frame(&mut self, frame_num: u64) -> &mut GameClockBuilder {
        self.start_frame = frame_num;
        self
    }
    /// Set the initial clock multiplier.
    ///
    /// Defaults to `1.0`.
    pub fn clock_multiplier(&mut self, multiplier: f64) -> &mut GameClockBuilder {
        self.clock_multiplier = multiplier;
        self
    }
    /// Construct a `GameClock` object with the set parameters.
    pub fn build(&self) -> GameClock {
        let start_game_time = GameTime {
            frame_wall_time: self.start_wall_time,
            frame_game_time: self.start_game_time,
            elapsed_game_time: FloatDuration::zero(),
            elapsed_wall_time: FloatDuration::zero(),
            frame_number: self.start_frame,
        };

        GameClock {
            last_frame_time: start_game_time,
            start_wall_time: self.start_wall_time,
            total_game_time: time::Duration::new(0, 0),
            current_frame: self.start_frame,
            clock_multiplier: self.clock_multiplier,
        }
    }
}

impl Default for GameClockBuilder {
    fn default() -> GameClockBuilder {
        GameClockBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use step;

    #[test]
    fn test_clock_construct() {
        let clock = GameClock::new();
        assert_eq!(clock.clock_multiplier(), 1.0);
        assert_eq!(
            clock.last_frame_time().elapsed_game_time(),
            FloatDuration::zero()
        );

        let clock2 = GameClockBuilder::new()
            .clock_multiplier(5.0)
            .start_frame(100)
            .build();

        assert_eq!(clock2.current_frame_number(), 100);
        assert_eq!(clock2.clock_multiplier(), 5.0);
        assert_eq!(
            clock2.last_frame_time().elapsed_game_time(),
            FloatDuration::zero()
        );
        assert_eq!(
            clock2.last_frame_time().elapsed_wall_time(),
            FloatDuration::zero()
        );
        assert!(clock2.last_frame_time().frame_wall_time() <= Local::now());

        let start_time = Local::today().and_hms(12, 0, 0);
        let clock3 = GameClockBuilder::new()
            .start_game_time(time::Duration::new(100, 0))
            .start_wall_time(start_time)
            .build();

        assert_eq!(clock3.current_frame_number(), 0);
        assert_eq!(
            clock3.last_frame_time().frame_game_time(),
            time::Duration::new(100, 0)
        );
        assert_eq!(clock3.last_frame_time().frame_wall_time(), start_time);

        assert_eq!(GameClock::default().current_frame_number(), 0);
        assert_eq!(
            GameClock::default().last_frame_time().frame_game_time(),
            time::Duration::new(0, 0)
        );
    }

    #[test]
    fn test_clock_tick() {
        let mut clock = GameClock::new();
        let time = clock.tick(&mut step::ConstantStep::new(FloatDuration::seconds(1.0)));
        assert_eq!(time.frame_number(), 1);
        assert_eq!(time.frame_game_time(), time::Duration::new(1, 0));
        assert!(time.frame_wall_time() > clock.start_wall_time());
        assert!(time.frame_wall_time() < Local::now());
        assert_eq!(time.elapsed_game_time(), FloatDuration::seconds(1.0));
        assert!(time.elapsed_wall_time() < FloatDuration::seconds(1.0));

        let time2 = clock.tick(&mut step::VariableStep::new());
        assert_eq!(time2.frame_number(), 2);
        assert!(time2.frame_wall_time() > time.frame_wall_time());
        assert_eq!(time2.elapsed_game_time(), time2.elapsed_wall_time());
        assert!(time2.frame_game_time() > time.frame_game_time());
    }

    #[test]
    fn test_clock_tick_loop() {
        let dt = FloatDuration::milliseconds(50.0);
        let mut step = step::ConstantStep::new(dt);
        let mut clock = GameClock::default();

        for x in 0..10 {
            let frame_time = clock.tick(&mut step);

            assert_eq!(frame_time.frame_number(), x + 1);
            assert_eq!(frame_time.elapsed_game_time(), dt);
            assert_eq!(
                frame_time.frame_game_time(),
                time::Duration::new(0, (50000000 * (x + 1)) as u32)
            );
        }

        let frame_time = clock.last_frame_time();
        assert_eq!(
            FloatDuration::from_std(frame_time.frame_game_time()),
            FloatDuration::seconds(0.5)
        );
        assert!(frame_time.frame_wall_time() > clock.start_wall_time());
    }
}
