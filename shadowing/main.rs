// Shadowing, this is not the same as marking a var as mutable

fn main() {
    let x = 5;

    let x = x + 1; // This second (new) x will now overshadow the original x

    // let x = 10; This will cause an error because x needs to be MUTABLE for total reassignment
    // Shaowing is playing off of the original variable, see how it's = x + 1 or = x * 2 ...

    let x = x + 2; // Now this third x overshadows the second, should be value 8 rn

    {
        let x = x * 2; // Now this fourth x overshadows the third
        println!("The value of x in the inner scope is {x}");
        // Notice how you can just put the var in the {}
    }

    let x = x + 10; // This will overshadow the third x, not the fourth since it's in an inner scope
    println!("The value of x at the end is {x}");

    // Shadowing can let you change the data type as well, as long as you ref the original var.
    let spaces = "     "; // orignally a string
    let spaces = spaces.len(); // now an int
    println!("The amount of spaces is {spaces}");
}