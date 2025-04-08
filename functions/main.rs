// In Rust you can have functions either before or after the main which is COOL!
fn hello_rust() {
    println!("Hello, Rust!");
}

// Main can be anywhere in the program, it's where the computer starts
fn main() {
    hello_world();
    hello_rust();
    tell_height(182); // throws a parameter to this function
    human_id("Ashton", 21, 180.327); // Throwing multiple param.'s

    // Function inside of main, expressive code block
    // Expression: Anything that returns a value i.e. 5, true & false, a fn add(3,4)
    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // This is the value being EXPRESSED, it'll be returned
    }; // notice the ; at the end of this function
    println!("Result is: {}", _x);

    // Fn that returns the sum, output assigned to a var.
    let y = add(4, 20);
    println!("Value of y from fn is: {}", y);
    println!("Function in the prinln: {}", add(7, 98));

    // For BMI
    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi = calculate_BMI(weight, height);
    println!("Your BMI is: {:.2}", bmi); // Display two decimal points
}

fn hello_world() {
    println!("Hello, World!");
}

// This function is expecting a positing integer, anything else makes an error
// The parameter is then assgined as the variale height
fn tell_height(height: u32) {
    println!("My height is {} cm", height);
}

// Same idea as above but using more param.'s
fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old and my height is {} cm", name, age, height);
}

// Function returning a value
// The -> i32 means that it'll return a data type of integer 32 bits
fn add(a: i32, b: i32) -> i32 {
    a + b // since it's a return value, don't put ;
}

// Statement: Anything that doesn't return a value
// Control flow statements include: if/else and while
    // Body Mass Index = height(kg)/height(m)*2
fn calculate_BMI(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}