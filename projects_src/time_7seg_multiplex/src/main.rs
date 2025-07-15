use std::env;

mod seven_segment;
mod digital_clock;

use digital_clock::{DigitalClock, ClockFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = env::args().collect::<Vec<String>>();
    
    if args.len() < 3 || args[1] == "-h" || args[1] == "--help" {
        let usage_txt = format!(r#"Usage:
    {} [option] <value>

Options:
    -h, --help      print help and exit
    -f              clock format (12h or 24h)
"#, args[0]);

        println!("{}", usage_txt);
        return Ok(());
    }

    if args.len() == 3 {
        if args[1] == "-f" {

            let clk_fmt = match args[2].as_str() {
                "12h" => ClockFormat::TwelveHour,
                "24h" => ClockFormat::TwentyFourHour,
                _ => return Err(format!("Invalid clock format: {}", args[2]).into()),
            };
            
            let mut clock = DigitalClock::new(clk_fmt)?;
            
            println!("Starting digital clock in {} format. Press Ctrl+C to stop.", args[2]);
            
            clock.run()?;

        } else {
            return Err(format!("Unknown option: {}", args[1]).into());
        }
    }

    Ok(())
}