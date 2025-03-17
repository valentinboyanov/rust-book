use std::io;

fn main() {
    println!("Generate the nth Fibonacci number.");

    loop {
        println!("Nth position: ");

        let mut position = String::new();

        io::stdin()
            .read_line(&mut position)
            .expect("Failed to read line.");

        if position.trim().is_empty() {
            continue;
        }

        if position.trim() == "exit" {
            break;
        }

        let position: u128 = position
            .trim()
            .parse()
            .expect("Failed to parse position.");

        println!("The nth position {position} has the Fibonacci number {}.", fibonacci(position));
    }
}

fn fibonacci(p: u128) -> u128 {
    match p {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fibonacci(p - 1) + fibonacci(p - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(0, fibonacci(0));
        assert_eq!(1, fibonacci(1));
        assert_eq!(1, fibonacci(2));
        assert_eq!(2, fibonacci(3));
        assert_eq!(3, fibonacci(4));
        assert_eq!(5, fibonacci(5));
        assert_eq!(55, fibonacci(10));
        assert_eq!(610, fibonacci(15));
        assert_eq!(4181 , fibonacci(19));
    }

}
