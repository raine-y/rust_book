fn main() {
    println!("");
    println!("~ Functions ~");
    println!("");
    print!("See? look: ");
    another_function(); // see bottom
    third_function(5);
    print_labeled_measurement(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

fn third_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}