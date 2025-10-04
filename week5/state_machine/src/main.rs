use std::io::{self, Write};

fn main() {
    println!("State machine: Signup Wizard");

    let mut state = State::Start;

    loop {
        match state {
            State::Start => {
                println!("Welcome and let's begin your signup!");
                state = State::EnterName;
            }
            State::EnterName => {
                let name = input("Enter your name");
                if name.is_empty() {
                    println!("Name can't be empty");
                } else {
                    state = State::EnterEmail(name);
                }
            }
            State::EnterEmail(ref name) => {
                let email = input("Enter your email");
                if !email.contains("@") {
                    println!("Please enter a valid email id");
                } else {
                    state = State::Confirm {
                        name: name.to_string(),
                        email,
                    };
                }
            }
            State::Confirm { name, email } => {
                println!("Confirm your info:");
                println!("Name: {name}");
                println!("Email: {email}");
                let confirm = input("Is this correct? (yes/no): ");
                state = match confirm.as_str() {
                    "yes" => State::Complete,
                    "no" => State::EnterName,
                    _ => {
                        println!("Invalid choice.");
                        State::Confirm { name, email }
                    }
                }
            }
            State::Complete => {
                println!("Sign up complete");
                break;
            }
        }
    }
}

enum State {
    Start,
    EnterName,
    EnterEmail(String),
    Confirm { name: String, email: String },
    Complete,
}

fn input(prompt: &str) -> String {
    println!("{prompt}");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read buffer");
    buf.trim().to_string()
}
