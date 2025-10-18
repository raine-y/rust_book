fn main() {
    { // s is not valid here, since it's not yet declared
        let _a = "hello";   // s is valid from this point forward
        // do stuff with s
    }   // this scope is now over, and s is no longer valid
   
    let mut b = String::from("hello");
    b.push_str(", world!");
    println!("{b}");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
Note that we pass &s1 into calculate_length and, in its definition, 
we take &String rather than String. These ampersands represent 
references, and they allow you to refer to some value without taking 
ownership of it. 
*/