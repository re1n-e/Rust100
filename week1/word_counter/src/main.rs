use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;

fn count_word(file: &mut File) -> usize {
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        panic!("Err reading file: {err}");
    }
    contents.split_whitespace().count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <filepath>");
        return;
    }

    let file_path = &args[1];
    let mut file = match OpenOptions::new().read(true).open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file: {err}");
            return;
        }
    };
    println!("Word count: {}", count_word(&mut file));
}
