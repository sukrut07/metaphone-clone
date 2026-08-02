//! Basic example showing how to use the metaphone library.

use metaphone::{dm, double_metaphone};

fn main() {
    let input_str = "richard";
    let (primary, secondary) = double_metaphone(input_str);
    println!(
        "'richard' -> Primary: '{}', Secondary: '{}'",
        primary, secondary
    );

    let owned_string = String::from("aubrey");
    let (p2, s2) = dm(&owned_string);
    println!("'aubrey' -> Primary: '{}', Secondary: '{}'", p2, s2);
}
