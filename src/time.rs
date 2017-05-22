use std::time;

use chrono;

pub struct GameClock {
    wall_start_global: chrono::DateTime<chrono::Local>,
    wall_start_frame: chrono::DateTime<chrono::Local>,
    game_start_frame: chrono::Duration,
    game_time_total: chrono::Duration,
    current_frame: u64,
    clock_multiplier: f64,
}

pub struct GameTime {
    game_time_elapsed: chrono::Duration,
    game_time_total: chrono::Duration,
    frame_number: u64,
    real_time_elapsed: chrono::Duration,
}

impl GameClock {
    pub fn new() -> GameClock {
        GameClock {
            wall_start_global: chrono::Local::now(),
            wall_start_frame: chrono::Local::now(),
            game_start_frame: chrono::Duration::milliseconds(0),
            game_time_total: chrono::Duration::milliseconds(0),
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

        let time_elapsed = now.signed_duration_since(self.wall_start_frame);
        let game_time_elapsed = time_elapsed;
        let frame_number = self.current_frame;
        let game_time_total = self.game_time_total;
        //TODO: Multipliers
        
        self.current_frame += 1;
        self.wall_start_frame = now;
        self.game_start_frame = self.game_start_frame + game_time_elapsed;

        GameTime {
            game_time_elapsed,
            game_time_total,
            frame_number,
            real_time_elapsed: time_elapsed,
        }

    }
}

impl GameTime {
}
