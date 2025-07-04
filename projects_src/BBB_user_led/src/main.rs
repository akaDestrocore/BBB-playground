use std::{
    env, fmt::format, fs::OpenOptions, io::Write, path::PathBuf
};

/// Script entry point.
/// 
/// # Returns
/// 
///  Result<(), Box<dyn Error>>
fn main() -> Result <(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        let usage_txt: String = format!(r#"Usage: {} <led_num> <control_option> <value>
Valid led numbers: 1-3
Valid control options: brightness, trigger
Valid 'brightness' values: 0, 1
Valid 'trigger' values: heartbeat, timer, none, default-on, oneshot
        "#, args[0]);
        println!("{}", usage_txt);
    }

    if args[2] == "brightness" {

        let brightness = &args[3];
        if brightness != "1" && brightness != "0" { 
            return Err("Invalid brightness value".into());
        }

        let path_str = "/sys/class/leds";
        let mut path: PathBuf = PathBuf::from(path_str);
        path.push(format!("beaglebone:green:usr{}/brightness", args[1]));

        let mut f = OpenOptions::new()
            .write(true)
            .open(&path)?;

        f.write_all(brightness.as_bytes())?;
    }

    else if args[2] == "trigger" {
        let trigger = &args[3];
        if trigger != "heartbeat" && trigger != "timer" && trigger != "none" && trigger != "default-on" && trigger != "oneshot" { 
            return Err("Invalid trigger value".into());
        }
        let path_str = "/sys/class/leds";
        let mut path: PathBuf = PathBuf::from(path_str);
        path.push(format!("beaglebone:green:usr{}/trigger", args[1]));
        
        let mut f = OpenOptions::new()
            .write(true)
            .open(&path)?;
        
        f.write_all(trigger.as_bytes())?;
    }

    else{
        println!("Valid control options: brightness, trigger");
        return Err("Invalid option".into())
    }

    Ok(())
}