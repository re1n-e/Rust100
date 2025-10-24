use std::io::{self, Write};

fn main() {
    println!("Genric sorting algorithm");

    loop {
        println!("\nChoose a type to sort: ");
        println!("1. Integers");
        println!("2. Words");
        println!("3. Exit");

        let choice = input("Your choice");
        match choice.as_str() {
            "1" => {
                let raw = input("Entera comma-seperated integers:");
                let mut nums: Vec<i32> = raw
                    .split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();
                let (low, high) = (0, nums.len() - 1);
                quick_sort(&mut nums, low, high);
                println!("✅ Sorted: {:?}", nums);
            }
            "2" => {
                let raw = input("Enter comma-separated words: ");
                let mut words: Vec<String> = raw.split(',').map(|s| s.trim().to_string()).collect();
                let (low, high) = (0, words.len() - 1);
                quick_sort(&mut words, low, high);
                println!("✅ Sorted: {:?}", words);
            }
            "3" => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid Choice\n"),
        }
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mut pivot_index = low;

    for i in low..high {
        if arr[i] < arr[high] {
            arr.swap(i, pivot_index);
            pivot_index += 1;
        }
    }

    arr.swap(pivot_index, high);
    pivot_index
}

fn quick_sort<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot = partition(arr, low, high);
        if pivot > 0 {
            quick_sort(arr, low, pivot - 1);
        }
        quick_sort(arr, pivot + 1, high);
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
