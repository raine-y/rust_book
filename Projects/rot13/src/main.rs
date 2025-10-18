use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    println!("|~~~~~~~~~~~~~~~~~~~~|");
    println!("|~ ROT13 ENCRYPTION ~|");
    println!("|~~~~~~~~~~~~~~~~~~~~|");
    println!("");
    loop {
        println!("Would you like to encrypt a file or enter text?");
        print!("Enter f for file, t for text and q to quit: ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim() {
            "f" | "F" => rot13_file(),
            "t" | "T" => {
                print!("Enter text to encrypt: ");
                let mut encrypt_me = String::new();
                io::stdout().flush().unwrap();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                rot13_text(encrypt_me)
            }
            "q" | "Q" => std::process::exit(0),
            _ => {
                println!("Invalid entry");
                continue;
            }
        }
    }
}

fn rot13_file() {}

fn rot13_text(encrypt_me: String) {
    
}
