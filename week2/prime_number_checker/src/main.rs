use std::io;

fn check_prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    }

    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    println!("Hello and welcome to prime number checker!");
    println!("Please enter any number to check if it is prime or not:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let num = input
        .trim()
        .parse::<i32>()
        .expect("Failed to parse input as i32");

    if check_prime(num) {
        println!("The number {num} is prime ✅");
    } else {
        println!("The number {num} is not prime ❌");
    }
}
