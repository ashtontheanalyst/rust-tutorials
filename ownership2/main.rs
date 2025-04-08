// This example piggy backs on the information from ownership

fn main() {
    let s1 = String::from("RUST"); // s1 is the owner of the string/memory 'RUST'
    let s2 = s1; // s2 is not the owner of the memory and string
    // s1 is now invalid because the data moved from it

    // println!("{}", s1); This will equate to a compile time error since it's not the owner
    println!("{}", s2);
}