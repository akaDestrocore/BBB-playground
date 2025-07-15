use std::{
    error::Error,
    thread,
    time::{Duration, SystemTime},
};

use crate::seven_segment::{Digits, SevenSegmentDisplay, Segment};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClockFormat {
    TwelveHour,
    TwentyFourHour
}

pub struct DigitalClock {
    display: SevenSegmentDisplay,
    format: ClockFormat,
}

impl DigitalClock {
    pub fn new(format: ClockFormat) -> Result<Self, Box<dyn Error>> {
        let display = SevenSegmentDisplay::new()?;
        Ok(Self {display, format})
    }

    pub fn get_current_time(&self) -> (u8, u8){
        
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let seconds = now % (60 * 60 * 24);
        let hours = (seconds / (60 * 60)) as u8;
        let minutes = ((seconds % (60 * 60)) / 60) as u8;

        let display_hours = match self.format {
            ClockFormat::TwelveHour => {
                if hours == 0 { 12 }
                else if hours > 12 { hours - 12 } 
                else { hours }
            },
            ClockFormat::TwentyFourHour => hours,
        };

        (display_hours, minutes)
    }

    pub fn display_time(&mut self) -> Result<(), Box<dyn Error>> {
        let (hours, minutes) = self.get_current_time();

        let digits = [
            hours / 10,
            hours % 10,
            minutes / 10,
            minutes % 10
        ];

        let display_digits = [Digits::D_1, Digits::D_2, Digits::D_3, Digits::D_4];

        for (i, &dig_pos) in display_digits.iter().enumerate() {
            if dig_pos == Digits::D_3 {
                self.display.set_segment(Segment::DP, gpiocdev::line::Value::Active)?;
                thread::sleep(Duration::from_millis(1));
            }
            self.display.enable_digit(dig_pos)?;
            self.display.set_digit(digits[i])?;
            thread::sleep(Duration::from_millis(3));
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.display_time()?;
        }
    }
}