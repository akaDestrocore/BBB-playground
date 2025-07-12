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

const GPIO_SEG_A: u32 = 546;
const GPIO_SEG_B: u32 = 547;
const GPIO_SEG_C: u32 = 549;
const GPIO_SEG_DP: u32 = 548;
const GPIO_SEG_D: u32 = 525;
const GPIO_SEG_E: u32 = 524;
const GPIO_SEG_F: u32 = 634;
const GPIO_SEG_G: u32 = 526;

pub struct SevenSegmentDisplay {
    lines: HashMap<u32, (u32, Request)>,
}

impl SevenSegmentDisplay {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        
        let mut lines = HashMap::new();
        
        let pins = [
            GPIO_SEG_A, GPIO_SEG_B, GPIO_SEG_C, GPIO_SEG_DP,
            GPIO_SEG_D, GPIO_SEG_E, GPIO_SEG_F, GPIO_SEG_G,
        ];

        for &pin in &pins {
            let (chip_path, offset) = Self::get_chip_offset(pin)?;
            
            let req = Request::builder()
                .on_chip(chip_path)
                .with_line(offset)
                .as_output(Value::Inactive)
                .request()?;
            
            lines.insert(pin, (offset, req));
        }

        Ok(Self { lines })
    }


    fn get_chip_offset(pin: u32) -> Result<(&'static str, u32), Box<dyn std::error::Error>> {
        
        match pin {
            512..=543 => Ok(("/dev/gpiochip0", pin - 512)),
            544..=575 => Ok(("/dev/gpiochip1", pin - 544)),
            576..=607 => Ok(("/dev/gpiochip2", pin - 576)),
            608..=639 => Ok(("/dev/gpiochip3", pin - 608)),
            _ => Err(format!("Unsupported pin: {}", pin).into()),
        }
    }

    pub fn set_segment(&mut self, pin: u32, value: Value) -> Result<(), Box<dyn std::error::Error>> {
        
        self.lines.get(&pin)
            .ok_or_else(|| format!("Pin {} not initialized", pin))?
            .1
            .set_value(self.lines[&pin].0, value)?;

        Ok(())
    }

    pub fn clear_all(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        
        let all_segments = [
            GPIO_SEG_A, GPIO_SEG_B, GPIO_SEG_C, GPIO_SEG_D,
            GPIO_SEG_DP, GPIO_SEG_E, GPIO_SEG_F, GPIO_SEG_G,
        ];
        
        for &segment in &all_segments {
            self.set_segment(segment, Value::Inactive)?;
        }

        Ok(())
    }

    pub fn set_digit(&mut self, digit: u8) -> Result<(), Box<dyn std::error::Error>> {
        
        self.clear_all()?;

        match digit {
            0 => {
                self.set_segment(GPIO_SEG_A, Value::Active)?;
                self.set_segment(GPIO_SEG_B, Value::Active)?;
                self.set_segment(GPIO_SEG_C, Value::Active)?;
                self.set_segment(GPIO_SEG_D, Value::Active)?;
                self.set_segment(GPIO_SEG_E, Value::Active)?;
                self.set_segment(GPIO_SEG_F, Value::Active)?;
            },
            1 => {
                self.set_segment(GPIO_SEG_B, Value::Active)?;
                self.set_segment(GPIO_SEG_C, Value::Active)?;
            },
            2 => {
                self.set_segment(GPIO_SEG_A, Value::Active)?;
                self.set_segment(GPIO_SEG_B, Value::Active)?;
                self.set_segment(GPIO_SEG_G, Value::Active)?;
                self.set_segment(GPIO_SEG_E, Value::Active)?;
                self.set_segment(GPIO_SEG_D, Value::Active)?;
            },
            3 => {
                self.set_segment(GPIO_SEG_A, Value::Active)?;
                self.set_segment(GPIO_SEG_B, Value::Active)?;
                self.set_segment(GPIO_SEG_G, Value::Active)?;
                self.set_segment(GPIO_SEG_C, Value::Active)?;
                self.set_segment(GPIO_SEG_D, Value::Active)?;
            },
            4 => {
                self.set_segment(GPIO_SEG_F, Value::Active)?;
                self.set_segment(GPIO_SEG_B, Value::Active)?;
                self.set_segment(GPIO_SEG_G, Value::Active)?;
                self.set_segment(GPIO_SEG_C, Value::Active)?;
            },
            5 => {
                self.set_segment(GPIO_SEG_A, Value::Active)?;
                self.set_segment(GPIO_SEG_F, Value::Active)?;
                self.set_segment(GPIO_SEG_G, Value::Active)?;
                self.set_segment(GPIO_SEG_C, Value::Active)?;
                self.set_segment(GPIO_SEG_D, Value::Active)?;
            },
            6 => {
                self.set_segment(GPIO_SEG_A, Value::Active)?;
                self.set_segment(GPIO_SEG_F, Value::Active)?;
                self.set_segment(GPIO_SEG_G, Value::Active)?;
                self.set_segment(GPIO_SEG_E, Value::Active)?;
                self.set_segment(GPIO_SEG_C, Value::Active)?;
                self.set_segment(GPIO_SEG_D, Value::Active)?;
            },
            7 => {
                self.set_segment(GPIO_SEG_A, Value::Active)?;
                self.set_segment(GPIO_SEG_B, Value::Active)?;
                self.set_segment(GPIO_SEG_C, Value::Active)?;
            },
            8 => {
                self.set_segment(GPIO_SEG_A, Value::Active)?;
                self.set_segment(GPIO_SEG_B, Value::Active)?;
                self.set_segment(GPIO_SEG_C, Value::Active)?;
                self.set_segment(GPIO_SEG_D, Value::Active)?;
                self.set_segment(GPIO_SEG_E, Value::Active)?;
                self.set_segment(GPIO_SEG_F, Value::Active)?;
                self.set_segment(GPIO_SEG_G, Value::Active)?;
            },
            9 => {
                self.set_segment(GPIO_SEG_A, Value::Active)?;
                self.set_segment(GPIO_SEG_B, Value::Active)?;
                self.set_segment(GPIO_SEG_C, Value::Active)?;
                self.set_segment(GPIO_SEG_D, Value::Active)?;
                self.set_segment(GPIO_SEG_F, Value::Active)?;
                self.set_segment(GPIO_SEG_G, Value::Active)?;
            },
            10 => {
                self.set_segment(GPIO_SEG_A, Value::Active)?;
                self.set_segment(GPIO_SEG_G, Value::Active)?;
                self.set_segment(GPIO_SEG_E, Value::Active)?;
                self.set_segment(GPIO_SEG_D, Value::Active)?;
                self.set_segment(GPIO_SEG_C, Value::Active)?;
            },
            _ => (),
        };

        Ok(())
    }

    pub fn set_decimal_point(&mut self, state: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.set_segment(GPIO_SEG_DP, if state { Value::Active } else { Value::Inactive })
    }
}

impl Drop for SevenSegmentDisplay {
    fn drop(&mut self) {
        let _ = self.clear_all();
    }
}