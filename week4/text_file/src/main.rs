use std::io::{self, BufRead, BufReader, Write};
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: cargo run -- <filepath>");
        return;
    }

    let filepath = &args[1];

    loop {
        println!("\n📝 Text File CRUD Menu:");
        println!("1. Create (overwrite)");
        println!("2. Read");
        println!("3. Update line");
        println!("4. Delete line");
        println!("5. Exit");

        match input("choose an option: ").as_str() {
            "1" => {
                let new_content = input("Enter new content");
                fs::write(filepath, new_content).expect("Failed to write to file");
                println!("File has been created (ovwritten)");
            }
            "2" => {
                if let Ok(file) = fs::File::open(filepath) {
                    println!("File contents: ");
                    for (i, line) in BufReader::new(file).lines().enumerate() {
                        println!("{}: {}", i, line.expect("Failed to read line"));
                    }
                } else {
                    println!("Failed to open file located at {filepath}");
                }
            }
            "3" => {
                let line_no = input("Line to update: ").parse::<usize>().unwrap_or(0);
                let new_text = input("New content: ");
                update_line(line_no, &new_text, filepath);
            }
            "4" => {
                let line_no = input("Line to delete: ").parse::<usize>().unwrap_or(0);
                delete_line(line_no, filepath);
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid Choice"),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn update_line(line_no: usize, new_text: &str, file_path: &str) {
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open file at {file_path}");
            return;
        }
    };

    let mut lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    if line_no < lines.len() {
        lines[line_no] = new_text.to_string();
        fs::write(file_path, lines.join("\n")).expect("Failed to write file");
        println!("Line {line_no} updated successfully!");
    } else {
        println!("Line {line_no} does not exist.");
    }
}

fn delete_line(line_no: usize, file_path: &str) {
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to open file at {file_path}");
            return;
        }
    };

    let mut lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    if line_no < lines.len() {
        lines.remove(line_no);
        fs::write(file_path, lines.join("\n")).expect("Failed to write file");
        println!("Line {line_no} deleted successfully!");
    } else {
        println!("Line {line_no} does not exist.");
    }
}
