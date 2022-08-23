use std::io; // importing io from the standard library, aka "std"
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret nuber is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // setting up a mutable variable for the user's input

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // The above code calls the read_lne method from stdin handle to get input from the user

    // The "&" indicates that the argument is a reference (references provide a way to let multiple)
    // parts of the code access one piece of data without needing to copy that data into memory
    // multiple times

    println!("You guessed: {}", guess);
}
