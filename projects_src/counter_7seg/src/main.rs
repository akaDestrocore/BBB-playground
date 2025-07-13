use std::{
    env, 
    fmt::format, 
    fs::OpenOptions, 
    io::Write,
    path::PathBuf,
    thread::sleep,
    time::{Duration, Instant},
};

mod seven_segment;
mod counter;

use seven_segment::{SevenSegmentDisplay};


/// Script entry point.
/// 
/// # Returns
/// 
///  Result<(), Box<dyn Error>>
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        let usage_txt: String = format!(r#"usage: {} <direction> <delay>
Valid directions : up, down,updown,random
Recommended delay range in ms : 0 to 1000"#, args[0]);
        println!("{}", usage_txt);
        
    } else {
        let delay_value: u64 = args[2].parse().unwrap();
        let mut counter = counter::Counter::new()?;

        if args[1] == "up" {
            counter.count_up(delay_value)?;

            return Ok(());
        } else if args[1] == "down" {
            counter.count_down(delay_value)?;

            return Ok(());
        } else if args[1] == "updown" {
            counter.count_updown(delay_value)?;
            return Ok(());
        } else if args[1] == "random" {
            counter.count_random(delay_value)?;
            return Ok(());
        }
    }

    Ok(())
}