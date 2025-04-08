// Primtive Data Types: int, float, bool, char

fn main() {
    // Signed int can be either neg or pos, unsigned int can only be positive
    // For int's, just use i32 for size, it's easiest
    let x: i32 = -42; // Assgins the number -42 to signed int of name x
    let y: u32 = 100; // Same but with an unsigned int
    println!("Signed Integer: {}", x); // Like an f print in python
    println!("Unsigned Integer: {}", y);

    let z = -20; // Not good practice but it works
    println!("Didn't assign a data value to {}", z);

    // Floats are either f32 or f64 depending on size, these are x.xxxx #'s
    let pi: f32 = 3.145;
    println!("This is pi: {}", pi);

    // Boolean values are only true, false
    let is_snowing: bool = true;
    println!("Is it snowing? {}", is_snowing);

    // Char, single code char type value a, b, Q, Z, ...
    let a: char = 'a';
    let b: char = 'b';
    println!("The first letter in the alpha is: {}, second is {}", a, b)
}