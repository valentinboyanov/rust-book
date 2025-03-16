use std::io;

fn main() {
    println!("Convert temperatures between Fahrenheit and Celsius.");

    println!("Please, select origin: F or C");

    let mut origin = String::new();

    io::stdin()
        .read_line(&mut origin)
        .expect("Failed to read line.");

    let origin: char = origin
        .trim()
        .parse()
        .expect("Failed to parse origin.");

    println!("Please, introduce temperature:");

    let mut temp_from = String::new();

    io::stdin()
        .read_line(&mut temp_from)
        .expect("Failed to read line.");

    let temp_from: f64 = temp_from
        .trim()
        .parse()
        .expect("Failder to parse temperature.");

    if origin == 'F' {
        let temp_to: f64 = (temp_from - 32.0) / (9.0/5.0);
        println!("{temp_from}°F => {temp_to:.2}°C",);
    } else if origin == 'C' {
        let temp_to: f64 = temp_from * (9.0/5.0) + 32.0;
        println!("{temp_from}°C => {temp_to:.2}°F",);
    } else {
        println!("Unknown origin.");
    }
}
