use std::{
    env, error::Error, fs::OpenOptions, io::Write, path::PathBuf
};

static USR_LED_NUMBER: u8 = 3;

/// Script entry point.
/// 
/// # Returns
/// 
///  Result<(), Box<dyn Error>>
fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        let man_txt: String = format!(r#"Usage: {args0} <control_option> <value>
valid control_options: brightness, trigger
valid 'brightness' values: 0,1
valid 'trigger' values: heartbeat, timer, none, oneshot, default-on
"#, args0 = args[0]);
        println!("{}", man_txt);
    }

    if args[1] == "trigger" {
        let res: Result<(), Box<dyn Error>> = process_trigger_value(&args[2]);
        res
    } else if args[1] == "brightness" {
        let res: Result<(), Box<dyn Error>> = process_brightness_value(&args[2]);
        return res
    } else if args[1] == "help" || args[1] == "info" {
        println!("This executable controls USER LED{lednum} of BBB", lednum = USR_LED_NUMBER);
        return Ok(());
    } else if args[1] == "info" {
        // write_info();
        println!("USER LED{lednum} is controlled by this executable", lednum = USR_LED_NUMBER);
        return Ok(());
    }else {
        println!("Valid control options: brightness, trigger");
        return Err("Invalid value".into());
    }

}

/// This function is used to pass the trigger value to the BBB user LED 
/// 
/// # Arguments
/// * `v` - The trigger value to be passed
/// 
/// # Returns
/// * `Ok(())` - If the trigger value is valid and successfully passed to the BBB user LED 
fn process_trigger_value(v: &str) -> Result<(), Box<dyn Error>> {
    match v {
        "heartbeat" | "timer" | "none" | "oneshot" | "default-on" => {
            let res: Result<(), Box<dyn Error>> = write_trigger_values(USR_LED_NUMBER, v);
            res
        }
        _ => Err("Invalid value. Valid trigger values : heartbeat, timer, none, oneshot, default-on".into())
    }
}

/// This function is used to pass the trigger value to the BBB user LED 
/// 
/// # Arguments
/// * `v` - The trigger value to be passed
///     
/// # Returns
/// * `Ok(())` - If the trigger value is valid and successfully passed to the BBB user LED
fn process_brightness_value(v: &str) -> Result<(), Box<dyn Error>> {
    match v {
        "0" | "1" => {
            let res = write_brightness_values(USR_LED_NUMBER, v.parse()?);
            res
        },
        _ => {
            return Err("Invalid value. Valid brightness values : 0 or 1".into())
        }
    }
}

/// This function is used to pass the trigger value to the BBB user LED 
/// 
/// # Arguments
/// * `led_no` - The number of the LED to be controlled
/// * `v` - The trigger value to be passed
///     
/// # Returns
/// * `Ok(())` - If the trigger value is valid and successfully passed to the BBB user LED
fn write_trigger_values(led_no: u8, v: &str) -> Result<(), Box<dyn Error>> {
    
    let mut path: PathBuf = PathBuf::from("/sys/class/leds");
    path.push(format!("beaglebone:green:usr{}/trigger", led_no));

    let mut f = OpenOptions::new()
        .write(true)
        .open(&path)?;
    
    f.write_all(v.as_bytes())?;

    Ok(())

}

/// This function is used to pass the brightness value to the BBB user LED 
/// 
/// # Arguments
/// * `led_no` - The number of the LED to be controlled
/// * `v` - 0 or 1
/// 
/// # Returns
/// * `Ok(())` - If the brightness value is valid and successfully passed to the BBB user LED  
fn write_brightness_values(led_no: u8, v: u8) -> Result<(), Box<dyn Error>> {
    
    let mut path: PathBuf = PathBuf::from("/sys/class/leds");
    path.push(format!("beaglebone:green:usr{}/brightness", led_no));

    let mut f = OpenOptions::new()
        .write(true)
        .open(&path)?;

    let byte: u8 = b'0' + v;
    f.write_all(&[byte])?;
    
    Ok(())
}