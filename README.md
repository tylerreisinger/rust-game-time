[game_time](https://docs.rs/game_time) 0.1.0
======================
[![Build Status](https://travis-ci.org/tylerreisinger/rust-game-time.svg?branch=master)](https://travis-ci.org/tylerreisinger/rust-game-time)
[![game_time on docs.rs][docsrs-image]][docsrs]
[![game_time on crates.io][crates-image]][crates]
[![codecov](https://codecov.io/gh/tylerreisinger/rust-game-time/branch/master/graph/badge.svg)](https://codecov.io/gh/tylerreisinger/rust-game-time)

[docsrs-image]: https://docs.rs/game_time/badge.svg
[docsrs]: https://docs.rs/game_time/0.1.0/
[crates-image]: https://img.shields.io/crates/v/game_time.svg
[crates]: https://crates.io/crates/game_time


Time handling for games and other real-time simulations.

Provides types and helpers for dealing with time in games. `game_time` allows for
easy decoupling of internal "game" time and external "wall" time, as well as tracking
frame rate.

Additionally, different kinds of time steps may be used, while also setting a multiplier
for speeding up/slowing down game time progression.

# Usage
Put this in your `Cargo.toml`:

```toml,ignore
[dependencies]
game_time = "0.1.0"
```

# Overview

`game_time` consists of 4 main types.

- [`GameClock`](clock/struct.GameClock.html) provides the basic time keeping for a simulation,
  tracking frames and returning a `GameTime` at the beginning of each.
- [`GameTime`](clock/struct.GameTime.html) provides the time information for each frame.
- [`FrameCount`](framerate/counter/struct.FrameCount.html) objects can be optionally used to track
  frame rate and other runtime statistics, utilizing swappable methods for computing the
  average fps.
- [`FrameRunner`](runner/struct.FrameRunner.html) combines a `GameClock` and `FrameCount` and
  makes it easy to run the simulation at a given frame rate.

For each frame, a [`TimeStep`](step/trait.TimeStep.html) is passed to `GameClock` in order
to advance the frame. This allows the frame rate to be changed at any time, and allows different
kinds of time steps (fixed, variable and a constant step are supported by default) to be used
based on what is most useful. Together, these objects combine to form a convenient but
flexible framework for time progression.

# Examples

Run a simulation with 50ms frames, without fps counting:

```rust
use game_time::GameClock;
use game_time::step;
use game_time::FloatDuration;

let mut clock = GameClock::new();
let end_time = FloatDuration::seconds(10.0);
let mut sim_time = clock.last_frame_time().clone();
let step = step::ConstantStep::new(FloatDuration::milliseconds(50.0));

while FloatDuration::from(sim_time.total_game_time()) < end_time {
    sim_time = clock.tick(&step);
    println!("Frame #{} at time={:?}", sim_time.frame_number(), sim_time.total_game_time());
}
```

Run a simulation at 30fps, sleeping if necessary to maintain the framerate:

```rust
use game_time::{GameClock, FrameCounter, FrameCount, FloatDuration};
use game_time::framerate::RunningAverageSampler;
use game_time::step;

let mut clock = GameClock::new();
let mut counter = FrameCounter::new(30.0, RunningAverageSampler::with_max_samples(60));
let end_time = FloatDuration::seconds(10.0);
let mut sim_time = clock.last_frame_time().clone();

while FloatDuration::from(sim_time.total_game_time()) < end_time {
    sim_time = clock.tick(&step::FixedStep::new(&counter));
    counter.tick(&sim_time);
    println!("Frame #{} at time={:?}", sim_time.frame_number(), sim_time.total_game_time());
}
```

