use std::thread;
use std::time;

use chrono;
use float_duration::{FloatDuration, TimePoint};

use framerate::{FrameCounter, FrameRateSampler, FrameCount};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeProgression {
    FixedStep,
    VariableStep,
}

#[derive(Debug, Clone)]
pub struct GameTime {
    frame_wall_time: chrono::DateTime<chrono::Local>,
    frame_game_time: time::Duration,
    elapsed_game_time: FloatDuration,
    elapsed_wall_time: FloatDuration,
    frame_number: u64,
}

#[derive(Debug, Clone)]
pub struct GameClock {
    start_wall_time: chrono::DateTime<chrono::Local>,
    frame_wall_time: chrono::DateTime<chrono::Local>,
    total_game_time: time::Duration,
    elapsed_game_time: FloatDuration,
    elapsed_wall_time: FloatDuration,
    current_frame: u64,
    clock_multiplier: f64,
    time_progression: TimeProgression,
}

impl GameClock {
    pub fn new() -> GameClock {
        let now = chrono::Local::now();

        GameClock {
            start_wall_time: now,
            frame_wall_time: now,
            total_game_time: time::Duration::new(0, 0),
            elapsed_game_time: FloatDuration::zero(),
            elapsed_wall_time: FloatDuration::zero(),
            current_frame: 0,
            clock_multiplier: 1.0,
            time_progression: TimeProgression::VariableStep,
        }
    }

    pub fn current_frame_index(&self) -> u64 {
        self.current_frame
    }
    pub fn start_wall_time(&self) -> chrono::DateTime<chrono::Local> {
        self.start_wall_time
    }
    pub fn frame_wall_time(&self) -> chrono::DateTime<chrono::Local> {
        self.frame_wall_time
    }
    pub fn frame_elapsed_time(&self) -> FloatDuration {
        chrono::Local::now()
            .float_duration_since(self.frame_wall_time)
            .unwrap()
    }

    pub fn clock_multiplier(&self) -> f64 {
        self.clock_multiplier
    }
    pub fn with_clock_multiplier(&mut self, val: f64) -> &mut GameClock {
        self.clock_multiplier = val;
        self
    }

    pub fn tick<C>(&mut self, counter: &mut C) -> GameTime
        where C: FrameCount + ?Sized
    {
        let frame_start = chrono::Local::now();

        self.current_frame += 1;

        let elapsed_wall_time = frame_start
            .float_duration_since(self.frame_wall_time)
            .unwrap();

        let elapsed_game_time = self.elapsed_game_time_from_wall_time(counter, elapsed_wall_time);
        let total_game_time = self.total_game_time + elapsed_game_time.to_std().unwrap();

        self.frame_wall_time = frame_start;
        self.total_game_time = total_game_time;
        self.elapsed_game_time = elapsed_game_time;
        self.elapsed_wall_time = elapsed_wall_time;

        let time = GameTime {
            frame_wall_time: frame_start,
            frame_game_time: total_game_time,
            elapsed_game_time,
            elapsed_wall_time,
            frame_number: self.current_frame,
        };

        counter.tick(&time);

        time
    }

    fn elapsed_game_time_from_wall_time<C>(&self,
                                           counter: &mut C,
                                           elapsed_wall_time: FloatDuration)
                                           -> FloatDuration
        where C: FrameCount + ?Sized
    {
        match self.time_progression {
            TimeProgression::FixedStep => counter.target_time_per_frame() * self.clock_multiplier, 
            TimeProgression::VariableStep => elapsed_wall_time * self.clock_multiplier, 
        }
    }

    pub fn sleep_remaining_via<C, F>(&mut self, counter: &C, f: F)
        where C: FrameCount + ?Sized,
              F: FnOnce(FloatDuration)
    {
        let remaining_time = counter.target_time_per_frame() -
                             chrono::Local::now()
                                 .float_duration_since(self.frame_wall_time)
                                 .unwrap();
        f(remaining_time)
    }

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
    pub fn frame_game_time(&self) -> time::Duration {
        self.frame_game_time
    }
    pub fn frame_wall_time(&self) -> chrono::DateTime<chrono::Local> {
        self.frame_wall_time
    }
    pub fn elapsed_game_time(&self) -> FloatDuration {
        self.elapsed_game_time
    }
    pub fn elapsed_wall_time(&self) -> FloatDuration {
        self.elapsed_wall_time
    }
    pub fn elapsed_time_since_frame_start(&self) -> FloatDuration {
        chrono::Local::now()
            .float_duration_since(self.frame_wall_time)
            .unwrap()
    }
    pub fn frame_number(&self) -> u64 {
        self.frame_number
    }
}
