use std::time;

use chrono;
use float_duration::{FloatDuration, TimePoint};

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

    pub fn tick(&mut self) -> GameTime {
        let frame_start = chrono::Local::now();

        self.current_frame += 1;

        let elapsed_wall_time = frame_start
            .float_duration_since(self.frame_wall_time)
            .unwrap();
        let elapsed_game_time = elapsed_wall_time * self.clock_multiplier;
        let total_game_time = self.total_game_time + elapsed_game_time.to_std().unwrap();

        self.frame_wall_time = frame_start;
        self.total_game_time = total_game_time;
        self.elapsed_game_time = elapsed_game_time;
        self.elapsed_wall_time = elapsed_wall_time;

        GameTime {
            frame_wall_time: frame_start,
            frame_game_time: total_game_time,
            elapsed_game_time,
            elapsed_wall_time,
            frame_number: self.current_frame,
        }
    }
}

impl Default for GameClock {
    fn default() -> GameClock {
        GameClock::new()
    }
}

impl GameTime {}
