use std::io;

fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2
}

fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2
}

fn multiply(num1: f64, num2: f64) -> f64 {
    num1 * num2
}

fn divide(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        panic!("Division by zero is not allowed");
    }
    num1 / num2
}

fn main() {
    println!("Simple calculator");
    println!("Available opeartions: +, -, *, /");
    println!("Enter your expression (e.g., 5 + 3):");

    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read expression");

    let tokens: Vec<&str> = expr.split_whitespace().collect();

    if tokens.len() != 3 {
        println!("Please provide a valide expression: num opeartor num");
        return;
    }

    let num1 = match tokens[0].parse::<f64>() {
        Ok(num) => num,
        Err(e) => panic!("Failed to parse num1: {e}"),
    };

    let num2 = match tokens[2].parse::<f64>() {
        Ok(num) => num,
        Err(e) => panic!("Failed to parse num1: {e}"),
    };

    let result = match tokens[1] {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "/" => divide(num1, num2),
        "*" => multiply(num1, num2),
        _ => panic!("Not a valid operator"),
    };

    println!("Result: {:.2}", result);
}
