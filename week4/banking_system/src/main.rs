use std::io::{self, Write};

struct Account {
    id: usize,
    name: String,
    balance: f64,
}

fn main() {
    let mut accounts: Vec<Account> = Vec::new();
    let mut next_id = 1;

    loop {
        println!("\n Banking System:");
        println!("1. Create Account");
        println!("2. View Balance");
        println!("3. Deposit");
        println!("4. Withdraw");
        println!("5. Exit");

        match input("Choose an option").as_str() {
            "1" => {
                let name = input("Account holder name:");
                let balance = input("Initial Deposit").parse::<f64>().unwrap_or(0.0);
                accounts.push(Account {
                    id: next_id,
                    name,
                    balance,
                });
                println!("✅ Account created with account id: {next_id}");
                next_id += 1;
            }
            "2" => {
                let id = input("Enter account id to view balance")
                    .parse::<usize>()
                    .unwrap_or(0);
                match accounts.iter().find(|acc| acc.id == id) {
                    Some(acc) => println!("Balance for {}: {:.2}", acc.name, acc.balance),
                    None => println!("❌ Account not found"),
                }
            }
            "3" => {
                let id = input("Enter account id to deposit")
                    .parse::<usize>()
                    .unwrap_or(0);
                let amount = input("Enter amount to deposit")
                    .parse::<f64>()
                    .unwrap_or(0.0);
                if let Some(acc) = accounts.iter_mut().find(|acc| acc.id == id) {
                    acc.balance += amount;
                    println!(
                        "✅ Deposited {:.2}. New balance: {:.2}",
                        amount, acc.balance
                    );
                } else {
                    println!("❌ Account not found");
                }
            }
            "4" => {
                let id = input("Enter account id to withdraw")
                    .parse::<usize>()
                    .unwrap_or(0);
                let amount = input("Enter amount to withdraw")
                    .parse::<f64>()
                    .unwrap_or(0.0);
                if let Some(acc) = accounts.iter_mut().find(|acc| acc.id == id) {
                    if acc.balance >= amount {
                        acc.balance -= amount;
                        println!(
                            "✅ Withdrawn {:.2}. New balance: {:.2}",
                            amount, acc.balance
                        );
                    } else {
                        println!("❌ Insufficient funds. Current balance: {:.2}", acc.balance);
                    }
                } else {
                    println!("❌ Account not found");
                }
            }
            "5" => {
                println!("👋 Exiting Banking System. Goodbye!");
                break;
            }
            _ => println!("⚠️ Unknown choice, please try again."),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{prompt} ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read input");
    buf.trim().to_string()
}
