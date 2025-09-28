use std::env;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Usage: cargo run -- <filepath>");
        return;
    }

    let filepath = &args[1];

    loop {
        
    }
}
