use std::io;


fn main() {

// **SCALAR DATA TYPES**
// Scalar types represent a single value.

    // INTEGER TYPES

    let guess: u32 = "42".parse().expect("Not a number!");
    // If the `: u32` type annotation isn't there, Rust will throw
    // an error that says the compiler needs more info
    println!("{guess}");

    // FLOATING POINT TYPES
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{x}, {y}");

// Note: unused vars throw a warning and Rust suggests prepending them with a "_"
// e.g. "x" would become "_x"

 
    // NUMERIC OPERATIONS

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
    
    println!("sum : {sum}, difference: {difference}, product: {product}, quotient: {quotient}, floored: {floored}, remainder: {remainder}");

    // BOOLEAN TYPE
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("{t}, {f}");

    // CHARACTER TYPE
    let c = 'z';
    let z: char = 'Z'; // with explicit type annocation
    let heart_eyed_cat = '😻';
    println!("{c}, {z}, {heart_eyed_cat}");


// **COMPOUND DATA TYPES**
// Compound types can group multiple values into one type.

    // TUPLE TYPE
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x, y, and z are: {x}, {y}, {z}");
    // This is called "destructuring" because it breaks the single
    // tuple into multiple parts.

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    // you can use a "." in between the tuple's name and the index you
    // want to retrieve
    println!("({five_hundred}, {six_point_four}, {one})");


    // ARRAY TYPE
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // i32 is the type of each element
    // After the semicolon, the number 5 indicates the array contains five elements
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    // how array indexing is done
    println!("{first}, {second}");

    let list = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    
    let element = list[index];

    println!("The value of the element at index {index} is: {element}");
    // If input is greater than length of list, this program will throw an error. This is an
    // example of Rust’s memory safety principles in action. In many low-level languages, this
    // kind of check is not done, and when you provide an incorrect index, invalid memory can
    // be accessed. Rust protects you against this kind of error by immediately exiting instead
    // of allowing the memory access and continuing. 🎉
}