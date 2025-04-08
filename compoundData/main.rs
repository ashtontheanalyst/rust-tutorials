// Compound Data Types: arrays, tuples, slices, and strings
// By default in Rust, every data type is immutable, you can't change it!

fn main() {
    // Arrays, fixed sized collection of elements of the same type
    let numbers: [i32; 5] = [1,2,3,4,5]; // array: [data type, number of values in array]
    println!("Number Array: {:?}", numbers); // the :? is needed in the ref {} to print the ENTIRE array
    
    // let mix = [1,2,"apple",true]; --> this would work as a tuple maybe...
    // This array won't work because it's a mix of data types

    
    // String SLICE array
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits); // returns the ENTIRE array as it is
    println!("First Fruit:  {}", fruits[0]); // returns the string value in the 0 position, first value
    println!("Second Fruit: {}", fruits[1]); // second string
    println!("Third Fruit:  {}", fruits[2]); // third string

   
    // Tuples are a fixed collection of different data types of the same size
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
        // to_string() turns Alice from a string slice to a string
    let my_mix_tuple = ("Doug", 8, true, [1,2,3,4,5]);
        // Don't have to declare each tuple value, just use case up to you
        // Can even have arrays in a tuple!
    println!("Human Tuple:  {:?}", human);
    println!("My Mix Tuple: {:?}", my_mix_tuple);

    
    /* Slices are basically just borrowing the data from something else, like taking
    some of the value from an array, it's referencing it (hence need for the &). 
    This means that it is immutable meaning it can't be change during runtime and 
    it doesn't have ownership of the data.

    EX: Imagine you have a big box of candy, and the box has 10 pieces of candy in
    it. But let’s say you don’t want to take the whole box to your friend; you 
    just want to give them a few pieces, like 3 candies from the box.

    Now, instead of taking those candies out and handing them over, you just let 
    your friend peek into the box and see those 3 candies. They can look at them, 
    but they don’t actually own the candies; they’re just borrowing a view of 
    those 3 pieces. */
    let number_slices: &[i32] = &[1,2,3,4,5]; //int slice array
    println!("Number Slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Tiger", "Girafe"]; //string slice array slice
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"Dune".to_string()]; // sting array slice
    println!("Book Slice: {:?}", book_slices);


    // Strings (growable, mutable, owned string type, allocated on the HEAP 
    // runtime, slow to make)
    let mut stone_cold: String = String::from("Hell, "); //stone_cold stored in Heap as a mutable 'mut' (changeable) string
    stone_cold.push_str("Yeah!"); // adds onto stone_cold string since its mutable
    println!("Stone Cold Says: {}", stone_cold);


    // vs. String Slices (&str) (fixed, immutable, ref to a string but not a 
    // string?, allocated on the stack in compile time, fast)
    let string: String = String::from("Hello, World!"); // this is a REAL string
    let slice: &str = &string; // Very similar to the concept of an original array and a slice of that array
    println!("Slice Value: {}", slice);

    let chopped_slice: &str = &string[0..5]; // Chopped up the string, see output
    println!("Chopped Slice Value: {}", chopped_slice);   
}