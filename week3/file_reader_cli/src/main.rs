use std::fs::{File, OpenOptions};
use std::io::Read;
use std::{env, io};

fn open_file(filepath: &str) -> std::io::Result<File> {
    OpenOptions::new().read(true).open(filepath)
}

fn search_file(buf: &String) -> std::io::Result<()> {
    println!("Enter the keyword to be searched");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    if buf.contains(input.trim()) {
        println!("Found the word");
    } else {
        println!("Failed to find the word");
    }
    Ok(())
}

fn read_file(filepath: &str) -> std::io::Result<()> {
    let mut file = open_file(filepath)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    println!("Press 1 if u want to search for a key word in the given file");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    match input.trim() {
        "1" => return search_file(&buf),
        _ => (),
    }
    for (i, lines) in buf.lines().enumerate() {
        println!("{} {}", i + 1, lines);
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <filepath>");
        return Ok(());
    }

    let filepath = &args[1];
    read_file(filepath)
}
