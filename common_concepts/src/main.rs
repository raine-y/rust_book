fn main() {
    // VARIABLES //////////////////////////////////////
    println!("");
    println!("~ Variables ~");
    println!("");
    let a = 4;
    println!("The value of a is: {a}");
    let mut b = 5;
    println!("The value of b is: {b}");
    b = 6;
    println!("The value of b is mutable, and: {b}");
    const SEA: u32 = 60 * 60 * 3;
    println!("The value of SEA is: {SEA}");
    // DATA TYPES //////////////////////////////////////
    // *SCALAR TYPES
    println!("");
    println!("~ Scalar Types ~");
    println!("");
    let x: u8 = 128;
    println!("The value of u8 is: {x}");
    let x: i8 = -127;
    println!("The value of i8 is: {x}");
    // i = signed u = unsigned, # = # of bits −(2n − 1) to 2n − 1 − 1
    // 8, 16, 32. 64, 128
    let x: f32 = 1.8;
    println!("The value of f32 is: {x}");
    // 32, 64
    // addition
    let sum = 5 + 10;
    println!("")
    // subtraction
    let difference = 95.5 - 4.3;
    println!("")
    // multiplication
    let product = 4 * 30;
    println!("")
    // division
    let quotient = 56.7 / 32.2;
    println!("")
    let truncated = -5 / 3; // Results in -1
    println!("")
    // remainder
    let remainder = 43 % 5;
    println!("")
    // *COMPOUND TYPES
}