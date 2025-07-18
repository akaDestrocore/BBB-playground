use std::collections::HashMap;

use gpiocdev::{
    line::Value,
    request::Request,
};


/*==================================================================================
BBB_P8_pins                         GPIO number            7Seg Display segment
===================================================================================
P8_7                                   GPIO-546                     A
P8_8                                   GPIO-547                     B
P8_9                                   GPIO-549                     C
P8_10                                  GPIO-548                     Dp
P8_11                                  GPIO-525                     D
P8_12                                  GPIO-524                     E
P8_14                                  GPIO-634                     F
P8_16                                  GPIO-526                     G
=================================================================================== */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Segment {
    A,
    B,
    C,
    DP,
    D,
    E,
    F,
    G,
}

impl Segment {
    fn gpio(self) -> u32 {
        match self {
            Segment::A => 546,
            Segment::B => 547,
            Segment::C => 549,
            Segment::DP => 548,
            Segment::D => 525,
            Segment::E => 524,
            Segment::F => 634,
            Segment::G => 526,
        }
    }
}

/// A struct representing a seven-segment display.
/// 
/// # Fields
/// * `lines` - A HashMap containing the lines of the display and their corresponding requests.
/// 
/// # Examples
/// ```
/// let display = SevenSegmentDisplay::new()?;
/// loop {
///    for i in 0..=10 {
///            self.display.set_digit(i)?;
///            thread::sleep(Duration::from_millis(delay));
///        }
///    }
/// ```
pub struct SevenSegmentDisplay {
    lines: HashMap<u32, (u32, Request)>,
}

impl SevenSegmentDisplay {
    /// Create a new instance of the SevenSegmentDisplay struct.
    /// 
    /// # Returns
    /// * `SevenSegmentDisplay` - A new instance of the SevenSegmentDisplay struct.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        
        let mut lines = HashMap::new();
        
        let segments = [
            Segment::A, Segment::B, Segment::C, Segment::DP,
            Segment::D, Segment::E, Segment::F, Segment::G,
        ];

        for &seg in &segments {

            let (chip_path, offset) = Self::get_chip_offset(seg.gpio())?;
            
            let req = Request::builder()
                .on_chip(chip_path)
                .with_line(offset)
                .as_output(Value::Inactive)
                .request()?;
            
            lines.insert(seg.gpio(), (offset, req));
        }

        Ok(Self { lines })
    }

    /// Get the chip offset for a given GPIO pin.
    /// 
    /// # Arguments
    /// * `pin` - The GPIO pin number.
    /// 
    /// # Returns
    /// * `Result<(&'static str, u32), Box<dyn std::error::Error>>` - A tuple containing the chip path and offset.
    fn get_chip_offset(pin: u32) -> Result<(&'static str, u32), Box<dyn std::error::Error>> {
        
        match pin {
            512..=543 => Ok(("/dev/gpiochip0", pin - 512)),
            544..=575 => Ok(("/dev/gpiochip1", pin - 544)),
            576..=607 => Ok(("/dev/gpiochip2", pin - 576)),
            608..=639 => Ok(("/dev/gpiochip3", pin - 608)),
            _ => Err(format!("Unsupported pin: {}", pin).into()),
        }
    }

    /// Set the value of a segment on the display.
    /// 
    /// # Arguments
    /// * `pin` - The GPIO pin number.
    /// * `value` - The value to set the segment to ( `Value::Active` or `Value::Inactive`). 
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - An error if the pin is not initialized.
    pub fn set_segment(&mut self, seg: Segment, value: Value) -> Result<(), Box<dyn std::error::Error>> {
        
        self.lines.get(&seg.gpio())
            .ok_or_else(|| format!("Pin {} not initialized", seg.gpio()))?
            .1
            .set_value(self.lines[&seg.gpio()].0, value)?;

        Ok(())
    }

    /// Clear all segments on the display.
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - An error if the pin is not initialized.
    pub fn clear_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        
        let segments = [
            Segment::A, Segment::B, Segment::C, Segment::D,
            Segment::DP, Segment::E, Segment::F, Segment::G,
        ];
        
        for &seg in &segments {
            self.set_segment(seg, Value::Inactive)?;
        }

        Ok(())
    }

    /// Set a digit on the display.
    /// 
    /// # Arguments
    /// * `digit` - The digit to set. Must be between 0 and 10 (inclusive).
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - An error if the pin is not initialized.
    pub fn set_digit(&mut self, digit: u8) -> Result<(), Box<dyn std::error::Error>> {
        
        self.clear_all()?;

        match digit {
            0 => {
                self.set_segment(Segment::A, Value::Active)?;
                self.set_segment(Segment::B, Value::Active)?;
                self.set_segment(Segment::C, Value::Active)?;
                self.set_segment(Segment::D, Value::Active)?;
                self.set_segment(Segment::E, Value::Active)?;
                self.set_segment(Segment::F, Value::Active)?;
            },
            1 => {
                self.set_segment(Segment::B, Value::Active)?;
                self.set_segment(Segment::C, Value::Active)?;
            },
            2 => {
                self.set_segment(Segment::A, Value::Active)?;
                self.set_segment(Segment::B, Value::Active)?;
                self.set_segment(Segment::G, Value::Active)?;
                self.set_segment(Segment::E, Value::Active)?;
                self.set_segment(Segment::D, Value::Active)?;
            },
            3 => {
                self.set_segment(Segment::A, Value::Active)?;
                self.set_segment(Segment::B, Value::Active)?;
                self.set_segment(Segment::G, Value::Active)?;
                self.set_segment(Segment::C, Value::Active)?;
                self.set_segment(Segment::D, Value::Active)?;
            },
            4 => {
                self.set_segment(Segment::F, Value::Active)?;
                self.set_segment(Segment::B, Value::Active)?;
                self.set_segment(Segment::G, Value::Active)?;
                self.set_segment(Segment::C, Value::Active)?;
            },
            5 => {
                self.set_segment(Segment::A, Value::Active)?;
                self.set_segment(Segment::F, Value::Active)?;
                self.set_segment(Segment::G, Value::Active)?;
                self.set_segment(Segment::C, Value::Active)?;
                self.set_segment(Segment::D, Value::Active)?;
            },
            6 => {
                self.set_segment(Segment::A, Value::Active)?;
                self.set_segment(Segment::F, Value::Active)?;
                self.set_segment(Segment::G, Value::Active)?;
                self.set_segment(Segment::E, Value::Active)?;
                self.set_segment(Segment::C, Value::Active)?;
                self.set_segment(Segment::D, Value::Active)?;
            },
            7 => {
                self.set_segment(Segment::A, Value::Active)?;
                self.set_segment(Segment::B, Value::Active)?;
                self.set_segment(Segment::C, Value::Active)?;
            },
            8 => {
                self.set_segment(Segment::A, Value::Active)?;
                self.set_segment(Segment::B, Value::Active)?;
                self.set_segment(Segment::C, Value::Active)?;
                self.set_segment(Segment::D, Value::Active)?;
                self.set_segment(Segment::E, Value::Active)?;
                self.set_segment(Segment::F, Value::Active)?;
                self.set_segment(Segment::G, Value::Active)?;
            },
            9 => {
                self.set_segment(Segment::A, Value::Active)?;
                self.set_segment(Segment::B, Value::Active)?;
                self.set_segment(Segment::C, Value::Active)?;
                self.set_segment(Segment::D, Value::Active)?;
                self.set_segment(Segment::F, Value::Active)?;
                self.set_segment(Segment::G, Value::Active)?;
            },
            10 => {
                self.set_segment(Segment::A, Value::Active)?;
                self.set_segment(Segment::G, Value::Active)?;
                self.set_segment(Segment::E, Value::Active)?;
                self.set_segment(Segment::D, Value::Active)?;
                self.set_segment(Segment::C, Value::Active)?;
            },
            _ => (),
        };

        Ok(())
    }

    /// Sets the value of decimal point segment.
    /// 
    /// # Arguments
    /// * `state` - The value to set the decimal point segment.
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - Result of the operation.
    pub fn set_decimal_point(&mut self, state: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.set_segment(Segment::DP, if state { Value::Active } else { Value::Inactive })
    }
}

/// Drop implementation to clear all segments when the object is dropped.
impl Drop for SevenSegmentDisplay {
    fn drop(&mut self) {
        let _ = self.clear_all();
    }
}