use std::io;

fn main() {
    println!("Enter choice: ");
    println!("Press 1 for Fahrenheit → Celsius");
    println!("Press 2 for Celsius → Fahrenheit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim() {
        "1" => fahrenheit_to_celsius(),
        "2" => celsius_to_fahrenheit(),
        _ => println!("Please enter correct choice number"),
    }
}

fn get_temp() -> f32 {
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature");

    match temp.trim().parse::<f32>() {
        Ok(val) => val,
        Err(e) => panic!("Failed to parse temp: {e}"),
    }
}

fn fahrenheit_to_celsius() {
    println!("Enter temperature in Fahrenheit: ");
    let f = get_temp();
    let c = (f - 32.0) * 5.0 / 9.0;
    println!("Temp in Celsius: {c}");
}

fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius: ");
    let c = get_temp();
    let f = c * 9.0 / 5.0 + 32.0;
    println!("Temp in Fahrenheit: {f}");
}
