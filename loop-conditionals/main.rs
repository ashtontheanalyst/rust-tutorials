// Conditional Statements, if-else statements

fn main() {
    /* Infinite loop
    loop {
        println!("Hello, World!");
    }
    */

    let mut counter = 0;

    let result = loop {
        counter += 1; // incriment the counter

        // stop when value is reached and exit program with returned value in the prinln
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}