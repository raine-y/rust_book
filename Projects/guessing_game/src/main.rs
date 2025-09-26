use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}");
   loop { //loops...
        println!("Your guess is: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is catch-all value, continue restarts loop
        };

        // Trim method eliminates white space, parse converts string to other type,  
        // : u32 tells rust data-type to parse to
        // +-+
        //If parse returns an Err Result variant because it couldn’t create a number from the string, 
        // the expect call will crash the game
    
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; //break the loop
            }
        }
    }
}
