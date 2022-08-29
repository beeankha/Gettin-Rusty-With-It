fn main() {
    println!("Hello, world!");

    another_function();
    parameters(5);
    print_labeled_measurement(5, 'h');
    expression();

}

fn another_function() {
    println!("Another function.");
}

fn parameters(x: i32) {
    println!("The value of x is: {x}");
}

// A note on function ordering: Rust doesn’t care where you define your functions,
// only that they’re defined somewhere in a scope that can be seen by the caller.

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
        // This is a block that, in this case, evaluates to 4. That value gets bound
        // to y as part of the `let` statement.
    };
    println!("The value of y is: {y}");
}