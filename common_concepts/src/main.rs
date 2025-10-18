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
    println!("~ The value of sum(+) is: {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("~ The value of difference(-) is: {difference}");
    // multiplication
    let product = 4 * 30;
    println!("~ The value of product(*) is: {product}");
    // division
    let quotient = 56.7 / 32.2;
    println!("~ The value of quotient(/) is: {quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("~ The value of truncated(/) is: {truncated}");
    // remainder
    let remainder = 43 % 5;
    println!("~ The value of remainder(%) is: {remainder}");
    let t = true;
    let f: bool = false;
    println!("{t} or {f}?");
    let string: char = 's';
    println!("String is {string}");
    // *COMPOUND TYPES
    println!("");
    println!("~ Compound Types ~");
    println!("");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("months[0]");
    // CONTROL FLOW //////////////////////////////////////
    println!("");
    println!("~ Control Flow ~");
    println!("");
    let number: i32 = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    println!("~ Loops");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    fn liftoff() {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
    liftoff();
}
