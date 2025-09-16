use rand::Rng;
use std::io;

fn guess_number(lives: &mut i32) {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..=100);

    println!("Guess the number between 1 and 100");

    while *lives > 0 {
        println!("You have {} lives left", *lives);
        println!("Enter your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        match guess.trim().parse::<i32>() {
            Ok(guess) => {
                if guess == num {
                    println!("🎉 Congratulations, you win!\n");
                    return;
                } else if guess > num {
                    println!("Too high!");
                } else {
                    println!("Too low!");
                }
                *lives -= 1;
            }
            Err(_) => {
                println!("Invalid input, please enter a number.\n");
                continue;
            }
        }
    }

    println!("💀 You lost! The number was {num}");
    println!("Better luck next time!\n");
}

fn main() {
    println!("Welcome to the guessing game\n");
    loop {
        println!("Please select your difficulty");
        println!("Easy:   9 Lives (Enter 1)");
        println!("Medium: 6 Lives (Enter 2)");
        println!("Hard:   3 Lives (Enter 3)\n");

        let mut difficulty = String::new();
        io::stdin()
            .read_line(&mut difficulty)
            .expect("Failed to read input");

        let mut lives = match difficulty.trim() {
            "1" => 9,
            "2" => 6,
            "3" => 3,
            _ => {
                println!("❌ Invalid choice, please try again.\n");
                continue;
            }
        };

        guess_number(&mut lives);

        println!("Enter 1 to play again, or any other key to quit:");
        let mut replay = String::new();
        io::stdin()
            .read_line(&mut replay)
            .expect("Failed to read input");

        if replay.trim() != "1" {
            println!("Thanks for playing! 👋");
            break;
        }

        println!("\n-------------------------------\n");
    }
}

