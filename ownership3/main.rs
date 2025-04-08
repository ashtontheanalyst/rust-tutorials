// This example piggy backs on the information from ownership and ownership2

fn main() {
    let s1 = String::from("RUST"); // s1 owns the string (the memory) that contains RUST
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}.", s1, len);
}

fn printLost(s: &String) {
    println!("{}", &s1);
    // s1 wasn't passed into this function, and we're outside of main, so
    // the program doesn't know what the value of s1 is
}

// The reference allows the function to only read the data not modify it
fn calculate_length(s: &String) -> usize {
    s.len()
}