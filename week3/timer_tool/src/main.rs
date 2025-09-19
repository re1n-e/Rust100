use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn get_timer_input() -> Option<(u64, u64, u64)> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Filed to read input");
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        return None;
    }
    let hour = parts[0].parse::<u64>().expect("Failed to parse hour");
    let minute = parts[1].parse::<u64>().expect("Failed to parse minute");
    let seconds = parts[2].parse::<u64>().expect("Failed to parse seconds");
    Some((hour, minute, seconds))
}

fn start_timer(hours: u64, minutes: u64, seconds: u64) {
    let total_seconds = hours * 60 * 60 + minutes * 60 + seconds;
    for i in (1..=total_seconds).rev() {
        let hrs = i / (3600);
        let min = (i % 3600) / (60);
        let sec = i % 60;

        print!(
            "\rThe time remaining: {:02} hours, {:02} minutes, {:02} seconds",
            hrs, min, sec
        );
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!();
}

fn main() {
    println!("Welcome to timer tool");
    println!("Enter the timer duration (format: hours minutes seconds)");

    let duration = match get_timer_input() {
        Some(dur) => dur,
        None => {
            println!("The input is not in required Hours minutes seconds format");
            return;
        }
    };

    println!(
        "Timer set for: {} hour {} minutes {} seconds",
        duration.0, duration.1, duration.2
    );

    start_timer(duration.0, duration.1, duration.2);

    println!("Time is up!");
}
