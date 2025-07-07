use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    time::Duration,
    thread::sleep
};

/// GPIO character device IOCTL definitions
const GPIO_GET_CHIPINFO_IOCTL: u32 = 0x8044b401;
const GPIO_GET_LINEINFO_IOCTL: u32 = 0xc048b402;
const GPIO_GET_LINEHANDLE_IOCTL: u32 = 0xc16cb403;
const GPIO_GET_LINEEVENT_IOCTL: u32 = 0xc030b404;

/// GPIO handle flags
const GPIOHANDLE_REQUEST_INPUT: u32 = 0x01;
const GPIOHANDLE_REQUEST_OUTPUT: u32 = 0x02;
const GPIOHANDLE_REQUEST_ACTIVE_LOW: u32 = 0x04;
const GPIOHANDLE_REQUEST_OPEN_DRAIN: u32 = 0x08;
const GPIOHANDLE_REQUEST_OPEN_SOURCE: u32 = 0x10;

/// GPIO event flags
const GPIOEVENT_REQUEST_RISING_EDGE: u32 = 0x01;
const GPIOEVENT_REQUEST_FALLING_EDGE: u32 = 0x02;
const GPIOEVENT_REQUEST_BOTH_EDGES: u32 = 0x03;

#[repr(C)]
struct GpioChipInfo {
    name: [i8; 32],
    label: [i8; 32], 
    lines: u32,
}

#[repr(C)]
struct GpioHandleRequest {
    line_offsets: [u32; 64],
    flags: u32,
    default_values: [u8; 64],
    consumer_label: [i8; 32],
    lines: u32,
    fd: i32,
}

#[repr(C)]
struct GpioEventRequest {
    line_offset: u32,
    handle_flags: u32,
    event_flags: u32,
    consumer_label: [i8; 32],
    fd: i32,
}

#[repr(C)]
struct GpioHandleData {
    values: [u8; 64],
}

#[derive(Debug)]
pub enum  GpioError {
    Io(std::io::Error),
    InvalidValue,
    ChipNotFound,
    LineBusy,
    PermissionDenied,
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

pub enum Edge {
    None,
    Rising,
    Falling,
    Both,
}

pub enum ActiveLevel {
    Low,
    High,
}

pub struct Gpio {
    pin: u16,
    chip_fd: Option<File>,
    handle_fd: Option<File>,
    event_fd: Option<File>,
    direction: Direction,
    active_level: ActiveLevel,
}

impl Gpio {
    pub fn new(pin: u16, _dir: Direction, _lvl: Level, _edge: Edge) -> GpioResult<Self> {
        
        let chip_fd = Self::open_chip(0)?;

        let mut gpio = Self { 
            pin, 
            chip_fd: Some(chip_fd), 
            handle_fd: None, 
            event_fd: None, 
            direction: _dir, 
            active_level: ActiveLevel::High 
        };

        gpio.set_direction(_dir)?;
        if matches!(_dir, Direction::Out) {
            gpio.set_value(_lvl)?;
        } 

        if !matches!(_edge, Edge::None) {
            gpio.set_edge(_edge)?;    
        }

        Ok(gpio)
    }

    pub fn set_direction(&mut self, _dir: Direction) -> GpioResult<()> {
        Ok(())
    }

    pub fn set_value(&mut self, _lvl: Level) -> GpioResult<()> {
        Ok(())
    }

    pub fn set_edge(&mut self, _edge: Edge) -> GpioResult<()> {
        Ok(())
    }
}