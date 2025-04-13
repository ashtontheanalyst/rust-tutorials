// Structs are used to name and package related values similar to tuples

fn main() {
    let rect: (i32,i32) = (200,500); // this is a tuple a rectangle with width and height

    // Struct, remind me a lot of a class in python
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Initializing a struct, i.e. making a user
    // This needs to be mutable in order to change any of the fields inside if need be
    let mut user1: User = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@gmail.com"),
        sign_in_count: 3,
    };
    
    println!("{}'s email is: {}", user1.username, user1.email);

    // Since user1 is mutable we can change the struct
    user1.email = String::from("noone@hotmail.com");
    println!("{}'s email after the change is: {}", user1.username, user1.email);


    // Return a struct from a function, notice the -> User for the return
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            email, // This is empty for now since the parameter is being passed into the function
            username, // Same, waiting param.
            sign_in_count: 1
        }
    }

    // Basically going to copy user1 data except we want a different email
    let user2 = User {
        username: String::from("anotherone"),
        email: String::from("another@gmail.com"), // email change as normal
        ..user1 // inherit everything else from user1
    };
    println!("{}'s email is {} and they're signed in {} time(s)", user2.username, user2.email, user2.sign_in_count);

    // tuple structs
    struct Color(i32, i32, i32);

    let black: Color = Color(0,0,0);
    let white = Color(255,255,255);

    // Unit-like struct, no attirbutes
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}