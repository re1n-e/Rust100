use std::{collections::HashMap, env, fs};
fn main() {
    println!("Hello and welcome to config file parser");
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: cargo run -- <filepath>");
        return;
    }

    let filepath = &args[1];

    match fs::read_to_string(&filepath) {
        Ok(content) => {
            let config = parse_config(&content);
            for (key, value) in config {
                println!("{key} {value}");
            }
        }
        Err(e) => println!("{e}"),
    }
}

fn parse_config(content: &String) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("#") || line.starts_with("[") {
            continue;
        }

        if let Some((key, value)) = line.split_once("=") {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    map
}
