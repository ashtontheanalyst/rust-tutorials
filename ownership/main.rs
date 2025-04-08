/* 
Ownership:
  - Every value has a single owner (every var. has one value, and it is its sole owner)

Rules: 
  1. Each value in rust has an owner
  2. There can only be one owner at a time
  3. When the owner goes out of scope, the value will be dropped

A reference is where you basically have a separate copy of the value, it is the exact
same as the value but it's separate from it like a pdf copy of a google doc
*/

fn main() {
    let s1 = String::from("RUST"); // s1 is the owner of that string, the value
    let len = calculate_length(&s1); // we're passing in a reference of s1
    println!("Length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}