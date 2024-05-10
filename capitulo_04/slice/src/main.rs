fn main() {
    let my_string = String::from("hello world");

    // works on Strings slices
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    // works on a String reference
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // works on slices of string literals
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // works on string literals reference
    let word = first_word(my_string_literal);

    // you can use slices on arrays too
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// A slice is a reference to a space in a saved value, it can be used in Strings, arrays and other
// data types
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
