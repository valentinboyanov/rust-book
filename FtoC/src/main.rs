use std::io;

// Improvements:
// - handle negative numbers
// - handle precision problems, eg: 123°F => 50.56°C, 50.56°C => 123.01°F
// - add unit tests
// - implement switch mode with second loop

fn main() {
    println!("Convert temperatures between Fahrenheit and Celsius.");

    loop {
        println!("Introduce temperature (eg. 123F or 65C):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim() == "exit" {
            break;
        }

        let mut temperature = input.trim().to_owned();

        let temp_unit = temperature.pop();

        let temp_from: f64 = temperature
            .trim()
            .parse()
            .expect("Failed to parse temperature.");

        if  temp_unit == Some('F') {
            let temp_to: f64 = (temp_from - 32.0) / (9.0/5.0);
            println!("{temp_from}°F => {temp_to:.2}°C",);
        } else if temp_unit == Some('C') {
            let temp_to: f64 = temp_from * (9.0/5.0) + 32.0;
            println!("{temp_from}°C => {temp_to:.2}°F",);
        } else {
            println!("Unknown unit.");
        }
    }
}
