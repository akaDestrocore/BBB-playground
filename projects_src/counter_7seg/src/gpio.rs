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

static mut CHIP_MANAGER: Option<GpioChipManager> = None;
static mut MANAGER_INITIALIZED: bool = false;

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

/// Methods for GPIO manager
impl GpioChipManager {
    /// Create new GPIO manager instance
    fn new() -> GpioResult<Self> {
        let mut manager: GpioChipManager = Self { 
            chips = HashMap::new(),
            pin_map = HashMap::new()
        };

        manager.discover_chips()?;
        Ok(manager)
    }

    /// Discover all available GPIO chips
    /// 
    /// Returns a list of all available GPIO chips
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

    /// Get the pin info for a given pin number.
    fn get_pin_info(&self, pin: u16) -> GpioResult<&PinInfo> {
        self.pin_map.get(&pin).ok_or(GpioError::PinNotFound(pin))
    }

    /// Get the chip for a given pin number.
    fn get_chip_for_pin(&self, pin: u16) -> GpioResult<&Chip> {
        let pin_info = self.get_pin_info(pin)?;
        self.chips.get(&pin_info.chip_path).ok_or(GpioError::PinNotFound(pin))
    }

    /// Get the chip for a given line number.
    fn list_pins(&self) -> Vec<(u16, &PinInfo)> {
        let mut pins: Vec<_> = self.pin_map.iter().map(|(k, v)| (*k, v)).collect();
        pins.sort_by_key(|(pin, _)| *pin);
        pins
    }
}

pub fn get_chip_manager() -> GpioResult<&'static GpioChipManager> {
    unsafe {
        if !MANAGER_INITIALIZED {
            CHIP_MANAGER = Some(GpioChipManager::new()?);
            MANAGER_INITIALIZED = true;
        }

        CHIP_MANAGER.as_ref().ok_or(GpioError::NotExported) 
    }
}

pub struct Gpio {
    pin: u16,
    direction: Direction,
    active_level: ActiveLevel,
    lines_input: Option<Lines<Input>>,
    lines_output: Option<Lines<Output>>,
}

impl Gpio {
    pub fn new(pin:u16, dir: Direction, lvl: Level, edge: Edge) -> GpioResult<Self> {
        
        let mut gpio = Self {
            pin,
            direction: dir,
            active_level: ActiveLevel::Low,
            lines_input: None,
            lines_output: None,
        };


        // TODO:
        gpio.set_direction(dir)?;
        gpio.set_value(lvl)?;

        Ok(gpio)
    }

    pub fn set_direction(&mut self, dir: Direction) -> GpioResult<()> {
        self.direction = dir;
        // TODO: 
        
        Ok(())
    }

    pub fn set_value(&mut self, lvl: Level) -> GpioResult<()> {
        // TODO:

        Ok(())
    }

    pub fn set_edge(&mut self, edge: Edge) -> GpioResult<()> {
        // TODO:

        Ok(())
    }

    pub fn read_value(&self) -> GpioResult<Level> {
        // TODO:

        Ok(Level::Low)
    }

    pub fn read_direction(&mut self) -> GpioResult<Direction> {
        // TODO:

        Ok(Direction::In)
    }

    pub fn read_edge(&mut self) -> GpioResult<Edge> {
        // TODO:

        Ok(Edge::None)
    }

    pub fn set_active_low(&mut self, active_low: ActiveLevel) -> GpioResult<()> {
        // TODO:

        Ok(())
    }

    pub fn read_active_low(&mut self) -> GpioResult<ActiveLevel> {
        // TODO:

        Ok(self.active_level)
    }

}