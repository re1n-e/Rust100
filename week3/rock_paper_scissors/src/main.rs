use rand::Rng;
use std::io;

#[derive(Clone, Debug)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

fn calculate_move_score(human_move: &Moves, computer_move: &Moves) -> i32 {
    match human_move {
        Moves::Rock => match computer_move {
            Moves::Paper => -1,
            Moves::Scissors => 1,
            Moves::Rock => 0,
        },
        Moves::Paper => match computer_move {
            Moves::Paper => 0,
            Moves::Scissors => -1,
            Moves::Rock => 1,
        },
        Moves::Scissors => match computer_move {
            Moves::Paper => 1,
            Moves::Scissors => 0,
            Moves::Rock => -1,
        },
    }
}

fn get_computer_move() -> Moves {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..=2) {
        0 => Moves::Rock,
        1 => Moves::Paper,
        _ => Moves::Scissors,
    }
}

fn main() {
    loop {
        println!("Hello and welcome to Rock, Paper & Scissors!");
        println!("First to five points wins.");
        let (mut human, mut computer) = (0, 0);

        while human < 5 && computer < 5 {
            let human_move = loop {
                println!("Please enter one of the following moves: [rock, paper, scissors]");

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                match input.to_ascii_lowercase().trim() {
                    "rock" => break Moves::Rock,
                    "paper" => break Moves::Paper,
                    "scissor" | "scissors" => break Moves::Scissors,
                    _ => {
                        println!("Invalid move. Try again.");
                        continue;
                    }
                }
            };

            let computer_move = get_computer_move();

            match calculate_move_score(&human_move, &computer_move) {
                1 => {
                    println!("You win this round!");
                    human += 1
                }
                -1 => {
                    println!("Computer wins this round!");
                    computer += 1
                }
                _ => println!("It's a tie!"),
            }

            println!("Current score: You {} - Computer {}\n", human, computer);
        }

        if human > computer {
            println!("🎉 Congrats, you won the game!");
        } else {
            println!("😢 You lose the game!");
        }

        println!("Press 1 to play again, or any other key to quit.");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.trim() != "1" {
            println!("Thnx for playing");
            break;
        }
    }
}
