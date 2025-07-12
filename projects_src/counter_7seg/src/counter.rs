use crate::seven_segment::SevenSegmentDisplay;
use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};

pub struct Counter {
    display: SevenSegmentDisplay,
}

impl Counter {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let display = SevenSegmentDisplay::new()?;
        Ok(Self { display })
    }

    pub fn count_up(&mut self, delay: u64) -> Result<(), Box<dyn std::error::Error>> {
        
        for i in 0..=10 {
            self.display.set_digit(i)?;
            thread::sleep(Duration::from_millis(delay));
        }

        Ok(())
    }
}