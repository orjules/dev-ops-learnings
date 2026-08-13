use std::env;

fn convert(command: &str, value: f32) -> Option<f32> {
    match command {
        "ft-m" => Some(value / 3.281),
        "m-ft" => Some(value * 3.281),
        _ => None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(&args);

    if args.len() != 3 {
        eprintln!("Expected 2 arguments, but got {}", args.len()-1);
        std::process::exit(1);
    }

    let command = args[1].as_str();
    let inital_value = args[2].parse::<f32>().unwrap_or_else(|_| {
        eprintln!("Could not parse '{}' as a number", args[2]);
        std::process::exit(2);
    });

    match convert(command, inital_value) {
        Some(a) => println!("{a}"),
        None => {
            println!("Could parse '{command}'");
            std::process::exit(3);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn basically_equals(value: f32, should_be: f32) -> bool {
        (value - should_be).abs() < 1e-4
    }

    #[test]
    fn feet_to_meters() {
        let result = convert("ft-m", 3.281).unwrap();
        assert!(basically_equals(result, 1.0));
    }

    #[test]
    fn meters_to_feet() {
        let result = convert("m-ft", 1.0).unwrap();
        assert!(basically_equals(result, 3.281));
    }

    #[test]
    fn unknown_command() {
        assert_eq!(convert("bogus", 0.0), None);
    }
}
