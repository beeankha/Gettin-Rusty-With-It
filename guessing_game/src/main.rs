use rand::Rng;
use std::cmp::Ordering;
use std::io; // importing io from the standard library, aka "std"

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // setting up a mutable variable for the user's input

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // The above code calls the read_lne method from stdin handle to get input from the user

        // The "&" indicates that the argument is a reference (references provide a way to let multiple)
        // parts of the code access one piece of data without needing to copy that data into memory
        // multiple times

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-invalid-input

        // The trim method on a String instance will eliminate any whitespace at the beginning and end, which
        // we must do to be able to compare the string to the u32, which can only contain numerical data.

        // parse() documentation: https://doc.rust-lang.org/std/primitive.str.html#method.parse

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // A "match" expression is made up of "arms"; an "arm" consists of a "pattern" to match against,
            // and the code that should be run if the value given to "match" fits that arm's pattern.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
