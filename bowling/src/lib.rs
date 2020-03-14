#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    throws: u8,
    score: u16,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            throws: 0,
            score: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.throws == 20 {
            return Err(Error::GameComplete);
        }

        self.throws += 1;
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        match self.throws == 20 {
            true => Some(self.score),
            false => None,
        }
    }
}
