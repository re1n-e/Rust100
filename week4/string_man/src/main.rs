use std::io;

fn prompt(msg: &str) -> String {
    println!("{msg}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn main() {
    println!("Welcome to string manipulation tool");

    loop {
        println!("\nChoose an operation:");
        println!("1. Reverse");
        println!("2. Uppercase");
        println!("3. Lowercase");
        println!("4. Trim");
        println!("5. Find Substring");
        println!("6. Replace Text");
        println!("7. Exit");

        let choice = prompt("Enter your choice: ");

        match choice.trim() {
            "1" => {
                let s = prompt("Enter a string: ");
                println!("Reverse: {}", s.chars().rev().collect::<String>());
            }
            "2" => {
                let s = prompt("Enter a string: ");
                println!("Uppercase: {}", s.to_uppercase());
            }
            "3" => {
                let s = prompt("Enter a string: ");
                println!("Lowercase: {}", s.to_lowercase());
            }
            "4" => {
                let s = prompt("Enter a string");
                println!("Trim: {}", s.trim());
            }
            "5" => {
                let s = prompt("Enter the main string: ");
                let sub = prompt("Enter the substring to find: ");
                if s.contains(sub.trim()) {
                    println!("Substring '{}' found!", sub);
                } else {
                    println!("Substring not found");
                }
            }
            "6" => {
                let s = prompt("Enter the main string: ");
                let old = prompt("Text to replace");
                let new = prompt("Replacement Text");
                println!("Result: {}", s.replace(old.trim(), new.trim()));
            }
            "7" => {
                println!("goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
