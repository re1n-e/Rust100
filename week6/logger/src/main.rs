use chrono::Local;
use std::fs::OpenOptions;
use std::io::{self, Write};

fn main() {
    println!("Logger Utility (werites to log.txt)");

    loop {
        println!("\nLog Levels: 1. INFO | 2. WARN | 3. ERROR| 4. EXIT");

        let choice = input("Select a level: ");

        match choice.as_str() {
            "1" => log_message("INFO"),
            "2" => log_message("WARN"),
            "3" => log_message("ERROR"),
            "4" => {
                println!("Exitting logger.");
                break;
            }
            _ => println!("Invalid Choice"),
        }
    }
}

fn log_message(level: &str) {
    let message = input(&format!("Enter {level} message: "));
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let entry = format!("[{}] {}: {}\n", timestamp, level, message);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("Unable to open log file");

    file.write_all(entry.as_bytes()).expect("Write failed");
    println!("Logged successfully.");
}

fn input(prompt: &str) -> String {
    println!("{prompt}");
    let mut buf = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read to buffer");
    buf.trim().to_string()
}
