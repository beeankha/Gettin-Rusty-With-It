// Here's how you can define and use a calculage_length function that has a reference
// to an object as a parameter instead of taking ownership of the value:
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // &s1 passes a reference to the value of s1
                                     // without taking ownership of it

    println!("The length of '{}' is {}.", s1, len);

    main_one();
    main_two();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// The ampersand (&) indicates that the type of the parameter s is a reference

// The opposite of referencing by using & is dereferencing, which is accomplished
// with the dereference operator, *.

// References allow you to refer to some value without taking ownership of it.
// With references, you can pass a value to some code, have that code operate on
// it, and then pass it back out again.

// Attempting to modify a borrowed value:
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }
// (the above will error out!)
// Just as variables are immutable by default, so are references. We're not allowed to
// modify something we have a reference to. Below is a fixed version of the above:
fn main_one() {
    let mut s = String::from("hello");

    change(&mut s); // &mut s passes a mutable reference to the value of s
                    // without taking ownership of it

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main_two() {
    let _reference_to_nothing = no_dangle();
    println!("{}", _reference_to_nothing);
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}