use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf, 
    thread::sleep,
    time::Duration
};

#[derive(Debug)]
pub enum GpioError {
    Io(io::Error),
    NotExported,
    InvalidValue
}

pub type GpioResult<T> = Result<T, GpioError>;

pub enum Direction {
    In,
    Out,
}

pub enum Level {
    Low,
    High,
}

pub struct Gpio {
    pin: u16,
    dir_f: File,
    value_f: File,
}

impl Gpio {
    /// Creates a new GPIO object for the given pin.
    /// 
    /// # Arguments
    /// * `pin` - The GPIO pin number.
    /// * `_dir` - The direction of the GPIO pin (input or output).
    /// * `_lvl` - The level of the GPIO pin (low or high).
    /// 
    /// # Returns
    /// * `gpio` - A new GPIO object.
    pub fn new(pin: u16, _dir: Direction, _lvl: Level) -> GpioResult<Self> {
        let path = PathBuf::from(format!("/sys/class/gpio/gpio{}", pin));

        Self::export(pin)?;
        
        sleep(Duration::from_millis(10));

        let value_file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(path.join("value"))
            .map_err(GpioError::Io)?;
        
        let dir_file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(path.join("direction"))
            .map_err(GpioError::Io)?;

        let mut gpio = Self {
            pin,
            dir_f: dir_file,
            value_f: value_file
        };

        gpio.set_direction(_dir)?;
        gpio.set_value(_lvl)?;

        Ok(gpio)
    }

    /// Sets the direction of the GPIO pin.
    /// 
    /// # Arguments
    /// * `_dir` - The direction of the GPIO pin.
    /// 
    /// # Returns
    /// * `GpioResult<()>` - A result indicating success or failure.
    pub fn set_direction(&mut self, _dir: Direction) -> GpioResult<()> {
        
        let d = match _dir {
            Direction::In => "in",
            Direction::Out => "out"
        };

        self.dir_f
            .seek(SeekFrom::Start(0))
            .map_err(GpioError::Io)?;
        self.dir_f
            .write_all(d.as_bytes())
            .map_err(GpioError::Io)?;
        self.dir_f
            .flush()
            .map_err(GpioError::Io)?;

        Ok(())
    }

    /// Sets the value of the GPIO pin.
    /// 
    /// # Arguments
    /// * `_lvl` - The value of the GPIO pin.
    /// 
    /// # Returns
    /// * `GpioResult<()>` - A result indicating success or failure.
    pub fn set_value(&mut self, _lvl: Level) -> GpioResult<()> {

        let v = match _lvl {
            Level::Low => "0",
            Level::High => "1"
        };

        self.value_f
            .seek(SeekFrom::Start(0))
            .map_err(GpioError::Io)?;
        self.value_f
            .write_all(v.as_bytes())
            .map_err(GpioError::Io)?;
        self.value_f.flush()
            .map_err(GpioError::Io)?;

        Ok(())
    }

    /// Exports the GPIO pin to the system.
    /// 
    /// # Returns
    /// * `GpioResult<()>` - A result indicating success or failure.
    pub fn export(pin: u16) -> GpioResult<()> {
        let path = PathBuf::from("/sys/class/gpio");

        let mut file = OpenOptions::new()
            .write(true)
            .open(path.join("export"))
            .map_err(GpioError::Io)?;

        match file.write_all(pin.to_string().as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::InvalidInput => Ok(()),
                    _ => Err(GpioError::Io(e)),
                }
            }
        }
    }

    /// Unexports the GPIO pin from the system.
    /// 
    /// # Returns
    /// * `GpioResult<()>` - A result indicating success or failure.
    pub fn unexport(&mut self, pin: u16) -> GpioResult<()> {
        let path = PathBuf::from("/sys/class/gpio");
        let mut file = OpenOptions::new()
            .write(true)
            .open(path.join("unexport"))
            .map_err(GpioError::Io)?;
        file.write_all(pin.to_string().as_bytes())
            .map_err(GpioError::Io)?;
        
        Ok(())
    }

    /// Reads the value of the GPIO pin.
    /// 
    /// # Returns
    /// * `GpioResult<Level>` - A result indicating the value of the GPIO pin.
    pub fn read_value(&mut self) -> GpioResult<Level> {
        let mut buffer =  [0u8; 1];

        self.value_f.seek(SeekFrom::Start(0))
            .map_err(GpioError::Io)?;
        self.value_f.read_exact(&mut buffer)
            .map_err(GpioError::Io)?;

        match buffer[0] {
            b'0' => Ok(Level::Low),
            b'1' => Ok(Level::High),
            _ => Err(GpioError::InvalidValue),
        }
    }
}

impl Drop for Gpio {
    fn drop(&mut self) {
        let _ = self.unexport(self.pin);
    }
}