// Constants, values that are bound to var.'s and not allowed to change
// They can be declared anywhere globally, doesn't have to be inside a function or main
fn main() {
    println!("Hello, world!");
    let x = 5;
    
    // const mut y = 10; This doesn't work because a const CANT BE MUTABLE, also missing data type i.e. i32
    // const y: i32 = 10; Should have capital letter
    const Y: i32 = 10;

    println!("The value of x is: {}", x);
    println!("The value of Y is: {}", Y);
    println!("The value of Pi is: {}", PI);
    println!("Three hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
}

const PI: f32 = 3.141592653; // See const can be declared outside of fn's or main
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;