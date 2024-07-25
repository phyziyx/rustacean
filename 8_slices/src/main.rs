// Self learning: do it by char for fun
// fn first_word(s: &String) -> usize {
//     for (i, c) in s.chars().enumerate() {
//         if c == ' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn first_word(s: &String) -> usize {
//     for (i, item) in s.bytes().into_iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_word(s: &str) -> &str {
    for (i, item) in s.bytes().into_iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let my_string = String::from("Hello, World!");
    const UNIQUE_STRING: &str = "Foo Bar";

    // Thanks to changing the `&String` to `&str`
    // allows us to use both, string slices and String objects
    let word = first_word(&UNIQUE_STRING[4..]);
    println!("First word: {word}");

    let word = first_word(&my_string);
    println!("First word: {word}");

    // Shadow variables!
    let word = first_word(&my_string[0..6]);
    println!("First word: {word}");
}
