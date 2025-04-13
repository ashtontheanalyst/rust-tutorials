// Very basic confitional while loop and for loop

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    } // countdown from 3 to 1

    println!("Sequence done");

    // Looping through a collection, array with a for loop
    let a: [i32; 5] = [1,2,3,4,5];
    for element in a {
        println!("{element}");
    }

    let b: [&str; 8] = ["do","re","me","fa","so","la","te","do"];
    println!("Learn your scales:");
    for element in b {
        println!("{element}");
    }
}