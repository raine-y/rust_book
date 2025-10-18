use std::io;
use std::io::Write;

fn main() {
    println!("~ Fibonacci Sequence ~");
    print!("Enter the degree to compute: ");
    let mut nth = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");
    let nth: u64 = nth.trim().parse().expect("Please type a number!");
    let result: u64 = fibonacci(nth);
    println!("F_{nth} of the fibonacci sequence is: {result}");
}

fn fibonacci(nth: u64) -> u64 {
    if nth <= 0 {
        return 0;
    } else if nth == 1 {
        return 1;
    } else {
        return fibonacci(nth - 1) + fibonacci(nth - 2);
    }
}
