use std::io;
use std::io::Write;

fn calculate_bmi(weight: f32, height: f32) {
    let bmi = weight / (height * height);

    if bmi < 18.5 {
        println!("Your BMI is {:.2}. Category: Underweight", bmi);
    } else if bmi < 25.0 {
        println!("Your BMI is {:.2}. Category: Normal weight", bmi);
    } else if bmi < 30.0 {
        println!("Your BMI is {:.2}. Category: Overweight", bmi);
    } else {
        println!("Your BMI is {:.2}. Category: Obesity", bmi);
    }
}

fn main() {
    println!("Hello and welcome to the BMI calculator!");

    print!("Please enter your height (in meters): ");
    io::stdout().flush().unwrap();
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read height");
    let height: f32 = height.trim().parse().expect("Please enter a valid number");

    if height <= 0.0 {
        println!("Height can't be zero or negative");
        return;
    }

    print!("Please enter your weight (in kilograms): ");
    io::stdout().flush().unwrap();
    let mut weight = String::new();
    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read weight");
    let weight: f32 = weight.trim().parse().expect("Please enter a valid number");

    if weight < 0.0 {
        println!("Weight can't be negative");
    }

    calculate_bmi(weight, height);
}
