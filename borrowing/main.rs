// Remember, only one variable can own one data value (see ownership ex's)
// References enable you to borrow values without taking ownership

// Immutable ref allow you to borrow without modification, Mutable does

fn main() {
    // remember that for single char var.'s its good prac to add a '_' before it
    let _x: i32 = 5;
    // let r: i32 = x; This is not good because it transfers ownership and data to
    // r and turns x invalid, try this:
    let _r: &i32 = &_x; // r is now an immutable reference of x
    println!("The value of _x is {}", _x);
    println!("The value of _r is {}", _r);

    // Making a mutable ref
    let mut _y: i32 = 10; // In order to make a mutable ref, you have to have an og mutable var
    let _z: &mut i32 = &mut _y; // Now this ref. is a mutable value of y
    *_z += 129; // both y and z will incrememnt this amount
    println!("The value of _y is {}", _y);
}