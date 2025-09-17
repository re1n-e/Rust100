use std::io;

fn generate_fibonacci(n: usize, dp: &mut Vec<usize>) -> usize {
    if dp[n] != usize::MAX {
        return dp[n];
    }

    if n == 0 {
        dp[n] = 0;
        return 0;
    }

    if n == 1 {
        dp[n] = 1;
        return 1;
    }

    dp[n] = generate_fibonacci(n - 1, dp) + generate_fibonacci(n - 2, dp);
    dp[n]
}

fn main() {
    println!("Welcome to the Fibonacci generator!");
    println!("Please enter a valid integer number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n = input
        .trim()
        .parse::<usize>()
        .expect("Failed to parse input, please provide a valid integer");

    let mut dp: Vec<usize> = vec![usize::MAX; n + 1];

    let result = generate_fibonacci(n, &mut dp);
    println!("Fibonacci({}) = {}", n, result);
}
