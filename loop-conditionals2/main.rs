// Nested loops
// To help readability we can label loops

fn main() {
    // outter loop
    let mut count = 0;
    // this loop is now labeled as counting_up thanks to the '
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        // inner loop
        loop {
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}