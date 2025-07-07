use std::{
    env, 
    fmt::format, 
    fs::OpenOptions, 
    io::Write,
    path::PathBuf
};

mod gpio;
mod seven_segment;

use seven_segment::{SevenSegmentDisplay};
use gpio::{ActiveLevel, Direction, Edge, Gpio, GpioError, GpioResult, Level};


/// Script entry point.
/// 
/// # Returns
/// 
///  Result<(), Box<dyn Error>>
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        let usage_txt: String = format!(r#"usage: {} <direction> <delay>
Valid direction : up, down,updown,random
Recommended delay range in ms : 0 to 1000"#, args[0]);
        println!("{}", usage_txt);
        
       let mut display1 = SevenSegmentDisplay::new().unwrap();
        display1.display_set_dp(gpio::Level::Low).unwrap();
        display1.display_digit(0).unwrap();
        
    } else {
        let delay_value: u32 = args[2].parse().unwrap();

        if args[1] == "up" {
            
            return Ok(());
        } else if args[1] == "down" {

            return Ok(());
        } else if args[1] == "updown" {

            return Ok(());
        } else if args[1] == "random" {

            return Ok(());
        }
    }

    Ok(())
}