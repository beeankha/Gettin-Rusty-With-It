// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }


// fn main() {

//     let mut spaces = "   ";
//     spaces = spaces.len();
//     // This will error out because we're not allowed to mutate a variable's type.
//     // (in this case, str -> int)

// }


fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}