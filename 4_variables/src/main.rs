// You can use `cargo new --name <project> <path>`
// to create a project with a different directory name

// Errors out when missing a type annotation
const NICE_CONSTANTS_BOZO: u32 = 1;
const AUTHOR_NAME: &str = "phyziyx";

fn main() {
    let x = 5;
    println!("[0] The value of x is: {x}");

    // Shadowing!
    let x = x + 1;

    // Shadowing! (again!)
    {
        let x = x * 2;
        println!("[1] The value of x is: {x}");
    }

    println!("[2] The value of x is: {x}");

    // Constants
    println!("The value of NICE_CONSTANTS_BOZO is: {NICE_CONSTANTS_BOZO}");
    println!("The value of AUTHOR_NAME is: {AUTHOR_NAME}");

    // Mutability
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // Errors out because spaces is immutable,
    // immutables can not change their types.
    let mut spaces = "    ";
    // spaces = spaces.len();
    println!("The value of spaces is: '{spaces}'");
}
