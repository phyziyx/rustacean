fn part_one() {
    // Ownership

    let s1 = String::from("hello");
    // Assigning 's1' is not allowed to 's2'
    // since in Rust, the samee variable can
    // not reference the same memory location
    // at once.
    // It _moves_ s1 to s2, and s1 is basically
    // now non-existant and unavailable for use.
    let s2 = s1;

    // To have access to both s1 and s2
    // you must use  `clone` in this case
    // or the property's size must be known
    // at the compile time (aka fixed size,
    // aka it is stored in the stack, rather
    // than the heap)

    println!("{s2}, world!");
}

fn part_two() {
    let s1 = String::from("hello");
    // Assigning 's1' is not allowed to 's2'
    // since in Rust, the samee variable can
    // not reference the same memory location
    // at once.
    // It _moves_ s1 to s2, and s1 is basically
    // now non-existant and unavailable for use.
    let s2 = s1.clone();

    // To have access to both s1 and s2
    // you must use  `clone` in this case
    // or the property's size must be known
    // at the compile time (aka fixed size,
    // aka it is stored in the stack, rather
    // than the heap)

    println!("{s1}, world! // {s2}, world!");
}

fn part_three() {
    // References and Borrowing with other fxns

    // self-task: create a function that takes
    // string as input and returns the length

    fn calculate_length(s: &str) -> usize {
        s.len()
    }

    const FIXED_STRING: &str = "Hi, My Name Is!";

    let dynamic_string = String::from("Slim Shady!");

    println!(
        "The string '{}' has length of {}",
        FIXED_STRING,
        calculate_length(FIXED_STRING)
    );

    println!(
        "The string '{}' has length of {}",
        dynamic_string,
        calculate_length(&dynamic_string)
    );
}

fn part_four() {
    // Listing 4-6

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let mut s = String::from("hello");

    change(&mut s);

    //

    let mut s = String::from("hello");

    // This piece of code is allowed here because there are no
    // "lingering" references to the data
    // s.push_str("🌍");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // It is not allowed here because there are references to the data
    // s.push_str("🌍");

    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    // takeaways:
    // there can be one mutable
    // and many immutable references
    // references must be valid
}

fn main() {
    // part_one();
    // part_two();
    // part_three();
    part_four();
}
