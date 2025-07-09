use std::{
    path::PathBuf,
    time::Duration,
    thread::sleep,
    collections::HashMap,
    fmt,
};

use gpiod::{
    Chip,
    Lines, 
    Options, 
    Input, 
    Output, 
    Active, 
    EdgeDetect
};

#[derive(Debug)]
pub enum GpioError {
    Io(std::io::Error),
    NotExported, 
    InvalidValue,
    PinNotFound(u16),
    LineInUse,
}

pub type GpioResult<T> = Result<T, GpioError>;

#[derive(Clone, Copy)]
pub enum Direction {
    In,
    Out,
}

#[derive(Clone, Copy)]
pub enum Level {
    Low,
    High,
}

#[derive(Clone, Copy)]
pub enum Edge {
    None,
    Rising,
    Falling,
    Both,
}

#[derive(Clone, Copy)]
pub enum ActiveLevel {
    Low,
    High,
}

#[derive(Debug, Clone)]
pub struct PinInfo {
    pub chip_path: PathBuf,
    pub line_offset: u32,
    pub chip_name: String,
    pub line_name: Option<String>,
}

struct GpioChipManager {
    chips: HashMap<PathBuf, Chip>,
    pin_map: HashMap<u16, PinInfo>,
}


impl From<std::io::Error> for GpioError {
    fn from(err: std::io::Error) -> Self {
        GpioError::Io(err)
    }
}

impl GpioChipManager {
    fn new() -> GpioResult<Self> {
        let mut manager: GpioChipManager = Self { 
            chips = HashMap::new(),
            pin_map = HashMap::new()
        };

        manager.discover_chips()?;
        Ok(manager)
    }

   fn discover_chips(&mut self) -> GpioResult<()> {
        let chip_paths = Chip::list_devices()?;
        let mut current_pin = 0u16;
        
        for path in chip_paths {
            let chip = Chip::new(&path)?;
            let num_lines = chip.num_lines();
            
            // Map lines to a corresponding pin numbers
            for line_offset in 0..num_lines {
                let line_info = chip.line_info(line_offset)?;
                let pin_info = PinInfo {
                    chip_path: path.clone(),
                    line_offset,
                    chip_name: chip.name().to_string(),
                    line_name: if line_info.name.is_empty() {
                        None
                    } else {
                        Some(line_info.name.clone())
                    },
                };
                
                self.pin_map.insert(current_pin, pin_info);
                current_pin += 1;
            }
            
            self.chips.insert(path, chip);
        }
        
        Ok(())
    }

    fn get_pin_info(&self, pin: u16) -> GpioResult<&PinInfo> {
        self.pin_map.get(&pin).ok_or(GpioError::PinNotFound(pin))
    }

}