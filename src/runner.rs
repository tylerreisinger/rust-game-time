use clock::GameClock;
use framerate::counter::FrameCount;

#[derive(Debug)]
pub struct FrameRunner {
    clock: GameClock,
    counter: Box<FrameCount>,
}

impl FrameRunner {
    pub fn new(clock: GameClock, counter: Box<FrameCount>) -> FrameRunner {
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
    pub fn counter(&self) -> &FrameCount {
        self.counter.as_ref()
    }
    pub fn counter_mut(&mut self) -> &mut FrameCount {
        self.counter.as_mut()
    }

    pub fn tick(&self) {}
}
