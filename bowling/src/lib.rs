#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: u16,
    throws: Vec<u16>,    
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            score: 0,
            throws: Vec::new(),            
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.throws.len() == 20 {
            return Err(Error::GameComplete);
        }

        self.throws.push(pins);
        
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        match self.throws.len() >= 20 {
            true => Some(self.score),
            false => None,
        }
    }
}
