use clock::{GameTime, GameClock};
use framerate::counter::FrameCount;

#[derive(Debug)]
pub struct FrameRunner<C: FrameCount> {
    clock: GameClock,
    counter: C,
}

impl<C> FrameRunner<C>
    where C: FrameCount
{
    pub fn new(clock: GameClock, counter: C) -> FrameRunner<C> {
        FrameRunner {
            clock: clock,
            counter: counter,
        }
    }

    pub fn clock(&self) -> &GameClock {
        &self.clock
    }
    pub fn clock_mut(&mut self) -> &mut GameClock {
        &mut self.clock
    }
    pub fn counter(&self) -> &C {
        &self.counter
    }
    pub fn counter_mut(&mut self) -> &mut C {
        &mut self.counter
    }
    pub fn tick(&mut self) -> GameTime {
        let time = self.clock.tick(&mut self.counter);
        time
    }
}
