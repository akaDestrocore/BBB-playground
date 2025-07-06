use std::{
    env, 
    fmt::format, 
    fs::OpenOptions, 
    io::Write,
    path::PathBuf
};

mod gpio;

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
    } else {
        let delay_value: u32 = args[2].parse().unwrap();

        if args[1] == "up" {
            // up count
            let res = counter_up(delay_value);
            match res {
                Ok(_) => println!("Up Counting done"),
                Err(e) => eprintln!("{}", e),
            }
        } else if args[1] == "down" {
            // down count
            // let res = counter_down(delay_value);
            Ok(());
        }

    }

    Ok(())
}

/// Counter up function.
///
/// # Arguments
///
/// * `delay` - delay in ms
///
/// # Returns
///
/// Result<(), Box<dyn Error>>  
fn counter_up(delay: u32) -> Result<(), Box<dyn std::error::Error>> {

    if init_gpios() < 0 {
        return Err("GPIO init failed".into());
    }
    Ok(())
}

