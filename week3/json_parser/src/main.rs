use std::env;
use std::fs::OpenOptions;
use std::io::Read;

fn get_file(filepath: &str) -> std::fs::File {
    match OpenOptions::new().read(true).open(filepath) {
        Ok(file) => file,
        Err(err) => panic!("Error opening file: {err}"),
    }
}

fn read_json(filepath: &str) -> String {
    let mut file = get_file(filepath);
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to read the file to the buffer");

    match serde_json::from_str::<serde_json::Value>(&buf) {
        Ok(json) => serde_json::to_string_pretty(&json).unwrap(),
        Err(err) => panic!("Invalid JSON: {err}"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run -- <filepath>");
        return;
    }

    let filepath = &args[1];
    let pretty_json = read_json(filepath);
    println!("{pretty_json}");
}
