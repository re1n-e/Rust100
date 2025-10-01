use std::{io, num::ParseFloatError};

fn main() {
    println!("Welcome to calculator");

    loop {
        println!("\n1. Add | 2. Divide | 3. Exit");

        let choice = input("Choose an option: ");

        match choice.as_str() {
            "1" => match parse_two_numbers() {
                Ok((a, b)) => println!("Result: {a} + {b} = {}", a + b),
                Err(e) => eprintln!("Error: {e}"),
            },
            "2" => match parse_two_numbers() {
                Ok((a, b)) => match divide(a, b) {
                    Ok(res) => println!("Result: {a} / {b} = {res}"),
                    Err(e) => eprintln!("Error: {e}"),
                },
                Err(e) => eprintln!("Error: {e}"),
            },
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{prompt}");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read buf");
    buf.trim().to_string()
}

fn parse_two_numbers() -> Result<(f64, f64), ParseFloatError> {
    let a = input("Enter first number").parse::<f64>()?;
    let b = input("Enter first number").parse::<f64>()?;
    Ok((a, b))
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Can't divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
