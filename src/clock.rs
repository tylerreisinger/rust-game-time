use chrono;

use super::duration::{FloatDuration, TimePoint};

pub struct GameTime {
    wall_time_elapsed: FloatDuration,
    game_time_elapsed: FloatDuration,
    game_time_total: FloatDuration,
    frame_number: u64,
}

pub struct GameClock {
    wall_start_global: chrono::DateTime<chrono::Local>,
    wall_start_frame: chrono::DateTime<chrono::Local>,
    game_start_frame: FloatDuration,
    game_time_elapsed: FloatDuration,
    wall_time_elapsed: FloatDuration,
    current_frame: u64,
    clock_multiplier: f64,
}

impl GameClock {
    pub fn new() -> GameClock {
        GameClock {
            wall_start_global: chrono::Local::now(),
            wall_start_frame: chrono::Local::now(),
            game_start_frame: FloatDuration::milliseconds(0.0),
            game_time_elapsed: FloatDuration::milliseconds(0.0),
            wall_time_elapsed: FloatDuration::milliseconds(0.0),
            current_frame: 0,
            clock_multiplier: 1.0,
        }
    }

    pub fn current_frame(&self) -> u64 {
        self.current_frame
    }
    pub fn game_start_time(&self) -> chrono::DateTime<chrono::Local> {
        self.wall_start_global
    }
    pub fn clock_multiplier(&self) -> f64 {
        self.clock_multiplier
    }
    pub fn set_clock_multiplier(&mut self, val: f64) -> &mut GameClock {
        self.clock_multiplier = val;
        self
    }

    pub fn tick(&mut self) -> GameTime {
        let now = chrono::Local::now();

        let time_elapsed = now.float_duration_since(self.wall_start_frame).unwrap();
        let game_time_elapsed = time_elapsed * self.clock_multiplier;
        let cur_game_time = self.game_start_frame + game_time_elapsed;
        let current_frame = self.current_frame;

        self.current_frame += 1;
        self.game_time_elapsed = game_time_elapsed;
        self.wall_time_elapsed = time_elapsed;
        self.game_start_frame = cur_game_time;
        self.wall_start_frame = now;
         
        GameTime {
            wall_time_elapsed: time_elapsed,
            game_time_elapsed,
            game_time_total: cur_game_time,
            frame_number: current_frame,
        }
    }
}

impl GameTime {
}

