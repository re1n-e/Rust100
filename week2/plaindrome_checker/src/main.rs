use std::io;

fn check_palindrome(input: &String) -> bool {
    let tokens: Vec<char> = input.trim().chars().collect();
    let mut low: usize = 0;
    let mut high = tokens.len().saturating_sub(1);
    while low < high {
        while low < high && !tokens[low].is_alphanumeric() {
            low += 1;
        }

        while low < high && !tokens[high].is_alphanumeric() {
            high -= 1;
        }
        if tokens[low].to_ascii_lowercase() != tokens[high].to_ascii_lowercase() {
            return false;
        }
        low += 1;
        high -= 1;
    }
    true
}

fn main() {
    println!("Welcome to palindrome checker!");
    println!("Please enter a string to check if it is a plainrome or not");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match check_palindrome(&input) {
        true => println!("Plaindrome string"),
        false => println!("Not a plaindrome string"),
    }
}
