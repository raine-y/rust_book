use std::io;
use std::io::Write;

fn main() {
    println!("~ Temperature Converter ~");
    print!("Please enter a temperature in Fahrenheit: ");
    let mut fahrenheit = String::new();
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");
    let fahrenheit_float: f32 = fahrenheit.trim().parse().expect("Please type a number!");
    let celsius: f32 =  ({fahrenheit_float} - 32.0) / (9.0/5.0);
    println!("{fahrenheit_float}°F is {celsius}°C");
}