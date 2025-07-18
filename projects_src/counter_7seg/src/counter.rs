use crate::seven_segment::SevenSegmentDisplay;
use rand::Rng;
use std::thread;
use std::time::Duration;

pub mod config{
    pub const MAX_DIGIT: u8 = 10;
    pub const MIN_DIGIT: u8 = 0;
}

/// A struct representing a counter that displays numbers on a seven-segment display.
/// 
/// # Fields
/// * `display` - A reference to the SevenSegmentDisplay struct.
/// 
/// # Examples
/// ```
/// let mut counter = counter::Counter::new()?;
/// counter.count_up(1000)?;
/// ```
pub struct Counter {
    display: SevenSegmentDisplay,
}

impl Counter {
    /// Create a new instance of the Counter struct.
    /// 
    /// # Returns
    /// * `Counter` - A new instance of the Counter struct.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let display = SevenSegmentDisplay::new()?;
        Ok(Self { display })
    }

    /// Count up from 0 to 10 (inclusive).
    /// 
    /// # Arguments
    /// * `delay` - The time delay between each count.
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - A Result indicating success or failure.
    pub fn count_up(&mut self, delay: u64) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            for i in config::MIN_DIGIT..=config::MAX_DIGIT {
                self.display.set_digit(i)?;
                thread::sleep(Duration::from_millis(delay));
            }
        }
    }

    /// Count down from 10 to 0 (inclusive).
    /// 
    /// # Arguments
    /// * `delay` - The time delay between each count.
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - A Result indicating success or failure.
    pub fn count_down(&mut self, delay: u64) -> Result<(), Box<dyn std::error::Error>> {
        
        loop {
            for i in (config::MIN_DIGIT..=config::MAX_DIGIT).rev() {
                self.display.set_digit(i)?;
                thread::sleep(Duration::from_millis(delay));
            }
        }
    }

    /// Count up and down from 0 to 10 (inclusive).
    /// 
    /// # Arguments
    /// * `delay` - The time delay between each count.
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - A Result indicating success or failure.
    pub fn count_updown(&mut self, delay: u64) -> Result<(), Box<dyn std::error::Error>> {
        
        loop {
            for i in config::MIN_DIGIT..=(config::MAX_DIGIT-1) {
                    self.display.set_digit(i)?;
                thread::sleep(Duration::from_millis(delay));
            }
            for i in ((config::MIN_DIGIT + 1)..=config::MAX_DIGIT).rev() {
                self.display.set_digit(i)?;
                thread::sleep(Duration::from_millis(delay));
            }
        }
    }

    /// Display random numbers in range from 0 to 10 (inclusive) at a specified delay.
    /// 
    /// # Arguments
    /// * `delay` - The time delay between each display.
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - A Result indicating success or failure.
    pub fn count_random(&mut self, delay: u64) -> Result<(), Box<dyn std::error::Error>> {
         let mut rng = rand::rng();
        
        loop {
            let random_digit = rng.random_range(0..10);
            self.display.set_digit(random_digit)?;
            thread::sleep(Duration::from_millis(delay));
        }
    }
}