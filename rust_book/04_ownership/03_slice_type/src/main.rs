fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]); // &my_string[..] is a slice of my_string
    println!("{}", word);

    let my_string_literal = "hello world";
    
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]); // &my_string_literal[..] is a slice of my_string_literal
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal); // "my_string_literal" is a slice of my_string_literal
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // "slice" is a slice of a
    println!("{:?}", slice);
    let slice = &a[..]; // "slice" is now the entire list
    println!("{:?}", slice);
}

// Original example without slicing:
fn first_word_without_slicing(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str { // &str is a string slice
    let bytes = s.as_bytes(); // as_bytes() converts a String to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iter() returns each element in a collection
                                                    // enumerate() returns a tuple of the index
                                                    // and the element
        if item == b' ' { // b' ' is a byte literal
            return &s[0..i]; // returns a slice of s from index 0 to i
        }
    }
    &s[..] // returns a slice of s from index 0 to the end of s
}
