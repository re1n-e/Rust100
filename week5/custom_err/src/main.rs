use std::error::Error;
use std::fmt;
use std::io::{self, Write};

fn main() {
    println!("Custom error handling: Square root calc");

    let input = prompt("Enter a number: ");
    match input.trim().parse::<f64>() {
        Ok(num) => match calculate_sqrt(num) {
            Ok(result) => println!("Sqrt of {num} is: {result}"),
            Err(e) => eprintln!("{e}"),
        },
        Err(err) => eprintln!("Invalid number format: {err}"),
    }
}

#[derive(Debug)]
enum MathError {
    NegativeInput,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MathError::NegativeInput => {
                write!(f, "Cannot calculate the square root of a negative number")
            }
        }
    }
}

impl Error for MathError {}

fn calculate_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeInput)
    } else {
        Ok(x.sqrt())
    }
}

fn prompt(prompt: &str) -> String {
    println!("{prompt}");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");
    buf.trim().to_string()
}
