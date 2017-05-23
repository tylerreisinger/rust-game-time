use std::thread;

use chrono;

use framerate::{FrameCounter, FrameRateSampler};
use duration::{FloatDuration, TimePoint};

pub struct GameTime {
    frame_start: chrono::DateTime<chrono::Local>,
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
        let now = chrono::Local::now();
        GameClock {
            wall_start_global: now,
            wall_start_frame: now,
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
    pub fn last_frame_game_time(&self) -> FloatDuration {
        self.game_start_frame
    }
    pub fn last_frame_wall_time(&self) -> chrono::DateTime<chrono::Local> {
        self.wall_start_frame
    }
    pub fn frame_elapsed_time(&self) -> FloatDuration {
        chrono::Local::now().float_duration_since(self.wall_start_frame).unwrap()
    }

    pub fn clock_multiplier(&self) -> f64 {
        self.clock_multiplier
    }
    pub fn set_clock_multiplier(&mut self, val: f64) -> &mut GameClock {
        self.clock_multiplier = val;
        self
    }
    pub fn with_clock_multiplier(mut self, val: f64) -> GameClock {
        self.clock_multiplier = val;
        self
    }

    pub fn sleep_remaining_via<S, F>(&mut self, counter: &FrameCounter<S>, f: F) 
        where S: FrameRateSampler,
              F: FnOnce(FloatDuration)
    {
        let remaining_time = counter.target_time_per_frame() 
            - chrono::Local::now().float_duration_since(self.wall_start_frame).unwrap();
        f(remaining_time)
    }

    pub fn sleep_remaining<S: FrameRateSampler>(&mut self, counter: &FrameCounter<S>) {
        self.sleep_remaining_via(counter, |rem| thread::sleep(rem.as_std().unwrap()))
    }

    pub fn tick(&mut self) -> GameTime {
        let now = chrono::Local::now();

        self.current_frame += 1;

        let time_elapsed = now.float_duration_since(self.wall_start_frame).unwrap();
        let game_time_elapsed = time_elapsed * self.clock_multiplier;
        let cur_game_time = self.game_start_frame + game_time_elapsed;
        let current_frame = self.current_frame;

        self.game_time_elapsed = game_time_elapsed;
        self.wall_time_elapsed = time_elapsed;
        self.game_start_frame = cur_game_time;
        self.wall_start_frame = now;
         
        GameTime {
            frame_start: now,
            wall_time_elapsed: time_elapsed,
            game_time_elapsed,
            game_time_total: cur_game_time,
            frame_number: current_frame,
        }
    }
}

impl GameTime {
    pub fn new(frame_start: chrono::DateTime<chrono::Local>, 
            wall_time_elapsed: FloatDuration, game_time_elapsed: FloatDuration,
            game_time_total: FloatDuration, frame_number: u64) -> GameTime 
    {
        GameTime {
            frame_start, 
            wall_time_elapsed, 
            game_time_elapsed,
            game_time_total,
            frame_number: frame_number,
        }

    }
    pub fn frame_start_time(&self) -> chrono::DateTime<chrono::Local> {
        self.frame_start
    }
    pub fn elapsed_time_since_frame_start(&self) -> FloatDuration {
        chrono::Local::now().float_duration_since(self.frame_start).unwrap()
    }
    pub fn elapsed_wall_time(&self) -> FloatDuration {
        self.wall_time_elapsed
    }
    pub fn elapsed_game_time(&self) -> FloatDuration {
        self.game_time_elapsed
    }
    pub fn total_game_time(&self) -> FloatDuration {
        self.game_time_total
    }
    pub fn frame_number(&self) -> u64 {
        self.frame_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use duration::{FloatDuration, TimePoint};

    #[test]
    fn construct_game_time() {
        let gt1 = GameTime::new(chrono::Local::now(), FloatDuration::milliseconds(20.0),
            FloatDuration::milliseconds(40.0), FloatDuration::seconds(2.04), 5);

        assert_eq!(gt1.frame_number(), 5);
        assert_eq!(gt1.elapsed_wall_time(), FloatDuration::milliseconds(20.0));
        assert_eq!(gt1.elapsed_game_time(), FloatDuration::milliseconds(40.0));
        assert_eq!(gt1.total_game_time(), FloatDuration::seconds(2.04));
        assert!(gt1.elapsed_time_since_frame_start() > FloatDuration::nanoseconds(0.0));
        assert!(gt1.elapsed_time_since_frame_start() < FloatDuration::seconds(1.0));
    }

    #[test]
    fn construct_game_clock() {
        let mut clock1 = GameClock::new();
        assert_eq!(clock1.current_frame(), 0);
        let game_time1 = clock1.tick();
        assert_eq!(clock1.current_frame(), 1);
        assert!(clock1.last_frame_game_time() > FloatDuration::zero());
        assert!(clock1.frame_elapsed_time() > FloatDuration::zero());

        assert_eq!(game_time1.frame_number(), 1);
        assert_eq!(game_time1.total_game_time(), game_time1.elapsed_game_time());

    }

    #[test]
    fn test_clock_multiplier() {
        let mut clock1 = GameClock::new().with_clock_multiplier(2.0);
        let game_time1 = clock1.tick();
        assert_eq!(clock1.clock_multiplier(), 2.0);
        assert_eq!(game_time1.elapsed_wall_time()*2.0, game_time1.elapsed_game_time());
    }
}
