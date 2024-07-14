fn main() {
    println!("Hello, world!");

    let sorted_array = [2, 4, 6, 8, 10];

    // Print the array
    println!("Sorted Array: {:?}", sorted_array);

    println!("Printing square: {:?}", sorted_array.map(|f| square(f)));
}

fn square(number: i32) -> i32 {
    number * number
}

// is the same as

// fn square(number: i32) -> i32 {
//     return number * number;
// }
