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

        let text = input.trim().to_owned();
        let (t_value, t_unit) = to_temperature(text);

        match t_unit {
            'F' => println!("{t_value:.2}°F => {:.2}°C", to_celsius(t_value)),
            'C' => println!("{t_value:.2}°C => {:.2}°F", to_fahrenheit(t_value)),
            _ => println!("Unknown temp unit."),
        }
    }
}

fn to_fahrenheit(temperature: f64) -> f64 {
    temperature * (9.0/5.0) + 32.0
}

fn to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) / (9.0/5.0)
}

fn to_temperature(mut text: String) -> (f64, char) {
    let t_unit = text.pop();

    let t_value: f64 = text
        .trim()
        .parse()
        .expect("Failed to parse temperature.");

    (t_value, t_unit.expect("Failed to parse unit."))
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

    #[test]
    fn test_to_temperature() {
        let text = String::from("123F");
        assert_eq!(to_temperature(text), (123.0, 'F'));
    }
}
