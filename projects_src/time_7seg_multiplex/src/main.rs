use std::fs;

fn main() {
    println!("Hello from Rust on BBB!");
    
    match fs::read_to_string("/sys/class/leds/beaglebone:green:usr3/trigger") {
        Ok(content) => println!("LED3 trigger options: {}", content.trim()),
        Err(e) => println!("Could not read LED file: {}", e),
    }
}