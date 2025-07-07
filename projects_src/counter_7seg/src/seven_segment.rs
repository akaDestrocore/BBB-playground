use std::{
    io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf, 
    collections::HashMap
};

use crate::gpio::{self, ActiveLevel, Direction, Edge, Gpio, GpioError, GpioResult, Level};

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
pub enum LedPins {
    P8_7SegA = 546,
    P8_8SegB = 547,
    P8_9SegC = 549,
    P8_10Dp = 548,
    P8_11SegD = 525,
    P8_12SegE = 524,
    P8_14SegF = 634,
    P8_16SegG = 526,
}

pub struct SevenSegmentDisplay {
    segments: HashMap<LedPins, Gpio>,
}

impl SevenSegmentDisplay {
    /// Creates a new SevenSegmentDisplay instance.
    /// 
    /// # Arguments
    /// * `led_pins` - A slice of LedPins enum values.
    /// 
    /// # Returns
    /// * `SevenSegmentDisplay` - A new SevenSegmentDisplay instance.
    pub fn new() -> GpioResult<Self> {

        let led_pins = [
            LedPins::P8_7SegA,
            LedPins::P8_8SegB,
            LedPins::P8_9SegC,
            LedPins::P8_10Dp,
            LedPins::P8_11SegD,
            LedPins::P8_12SegE,
            LedPins::P8_14SegF,
            LedPins::P8_16SegG,
        ];

        let mut segments = HashMap::new();

        for pin in led_pins.iter() {
            let gpio = Gpio::new(*pin as u16, Direction::Out, Level::Low, Edge::None)?;
            segments.insert(*pin, gpio);
            println!("Initialized pin {} successfully!", *pin as u16);
        }

        Ok(SevenSegmentDisplay { segments })
    }

    /// This function sets the value of a specific segment on the seven-segment display.
    /// 
    /// # Arguments
    /// * `segment` - The segment to set the value for.
    /// * `state` - The value to set the segment to. Can be either Level::Low or Level::High.
    /// 
    /// # Returns
    /// A `GpioResult` indicating the success or failure of the operation.
    pub fn set_segment(&mut self, segment: LedPins, state: Level) -> GpioResult<()> {
        
        if let Some(gpio) = self.segments.get_mut(&segment) {
            gpio.set_value(state)
        } else {
            Err(GpioError::NotExported)
        }
    }

    /// This function clears the display on the seven-segment display.
    /// 
    /// # Returns
    /// A `GpioResult` indicating the success or failure of the operation.
    pub fn clear_display(&mut self) -> GpioResult<()> {
        for gpio in self.segments.values_mut() {
            gpio.set_value(Level::Low);
        }
        Ok(())
    }

    /// This function displays a digit on the seven-segment display.
    /// 
    /// # Arguments
    /// * `digit` - The digit to display. Can be any value from 0 to 9.
    /// 
    /// # Returns
    /// A `GpioResult` indicating the success or failure of the operation.
    pub fn display_digit(&mut self, digit: u8) -> GpioResult<()> {
        self.clear_display();

        let digit_segs = match digit {
                0 => vec![
                    LedPins::P8_7SegA,
                    LedPins::P8_8SegB,
                    LedPins::P8_9SegC,
                    LedPins::P8_11SegD,
                    LedPins::P8_12SegE,
                    LedPins::P8_14SegF,
                ],
                1 => vec![
                    LedPins::P8_8SegB,
                    LedPins::P8_9SegC,
                ],
                2 => vec![
                    LedPins::P8_7SegA,
                    LedPins::P8_8SegB,
                    LedPins::P8_16SegG,
                    LedPins::P8_12SegE,
                    LedPins::P8_11SegD
                ],
                3 => vec![
                    LedPins::P8_7SegA,
                    LedPins::P8_8SegB,
                    LedPins::P8_16SegG,
                    LedPins::P8_9SegC,
                    LedPins::P8_11SegD
                ],
                4 => vec![
                    LedPins::P8_14SegF,
                    LedPins::P8_16SegG,
                    LedPins::P8_8SegB,
                    LedPins::P8_9SegC,
                ],
                5 => vec![
                    LedPins::P8_7SegA,
                    LedPins::P8_14SegF,
                    LedPins::P8_16SegG,
                    LedPins::P8_9SegC,
                    LedPins::P8_11SegD
                ],
                6 => vec![
                    LedPins::P8_7SegA,
                    LedPins::P8_14SegF,
                    LedPins::P8_16SegG,
                    LedPins::P8_12SegE,
                    LedPins::P8_9SegC,
                    LedPins::P8_11SegD
                ],
                7 => vec![
                    LedPins::P8_7SegA,
                    LedPins::P8_8SegB,
                    LedPins::P8_9SegC
                ],
                8 => vec![
                    LedPins::P8_7SegA,
                    LedPins::P8_8SegB,
                    LedPins::P8_9SegC,
                    LedPins::P8_11SegD,
                    LedPins::P8_12SegE,
                    LedPins::P8_14SegF,
                    LedPins::P8_16SegG
                ],
                9 => vec![
                    LedPins::P8_7SegA,
                    LedPins::P8_8SegB,
                    LedPins::P8_9SegC,
                    LedPins::P8_11SegD,
                    LedPins::P8_14SegF,
                    LedPins::P8_16SegG
                ],
                _ => return Err(GpioError::InvalidValue),
        };

        for segment in digit_segs {
            self.set_segment(segment, Level::High)?;
        }

        Ok(())
    }

    /// Set the decimal point segment to a given level.
    /// 
    /// # Arguments
    /// * `segment` - The segment to set.
    /// * `level` - The level to set the segment to.
    /// 
    /// # Returns
    /// * `GpioError` - An error if the segment is not exported.
    pub fn display_set_dp(&mut self, state: Level) -> GpioResult<()> {
        self.set_segment(LedPins::P8_10Dp, state)
    }

}