use std::io;

// Improvements:
// - handle negative numbers
// - handle precision problems, eg: 123°F => 50.56°C, 50.56°C => 123.01°F
// - implement switch mode with second loop
// - handle empty input

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

        match temp_unit {
            Some('F') => println!("{temp_from:.2}°F => {:.2}°C", to_celsius(temp_from)),
            Some('C') => println!("{temp_from:.2}°C => {:.2}°F", to_fahrenheit(temp_from)),
            _ => println!("Unknown unit."),
        }
    }
}

fn to_fahrenheit(temperature: f64) -> f64 {
    temperature * (9.0/5.0) + 32.0
}

fn to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) / (9.0/5.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_celsius() {
        assert_eq!(to_celsius(122.0), 50.0);
    }

    #[test]
    fn test_to_fahrenheit() {
        assert_eq!(to_fahrenheit(0.0), 32.0);
    }
}
