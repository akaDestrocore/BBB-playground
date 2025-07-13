use std::env;

mod seven_segment;
mod counter;

/// Organized variable configuration for the script.
pub mod config{
    pub const MAX_DELAY_MS: u64 = 9999;
    pub const DEFAULT_DELAY_MS: u64 = 1000;
}

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
        let mut delay_value: u64 = args[2].parse().map_err(|_|"Delay value must be a number")?;
        let mut counter = counter::Counter::new()?;
        if delay_value > config::MAX_DELAY_MS {
            println!("Delay value must be less than or equal to {}", config::MAX_DELAY_MS);
            delay_value = config::DEFAULT_DELAY_MS;
        }

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