# [Roguelike Tutorial -- In Rust](https://bfnightly.bracketproductions.com/rustbook/chapter_0.html) 

Learning Rust by creating a [roguelike](https://en.wikipedia.org/wiki/Roguelike)!

Code related to the book can be found on [GitHub](https://github.com/amethyst/rustrogueliketutorial).

* * *

## Notes

In the [Building for the Web](https://bfnightly.bracketproductions.com/rustbook/webbuild.html) chapter, the section entitled ["Building for the Web"](https://bfnightly.bracketproductions.com/rustbook/webbuild.html#building-for-the-web) is missing instructions in [**Step 1: Compile the program for WASM**](https://bfnightly.bracketproductions.com/rustbook/webbuild.html#step-1-compile-the-program-for-wasm).Before running
```bash
$ cargo build --release --target wasm32-unknown-unknown
```
... the following needs to be run _first_:
```bash
$ cargo init wasm32-unknown-unknown
$ cd wasm32-unknown-unknown
```

* * *

## Resources

* [Where to get `rustup`](https://rustup.rs/)
* The [full list of crates that are on offer in Cargo](https://crates.io/)

* * * 

## References / Things to Read

### Chapter 1: Hello Rust

* [`rltk` Crate Page](https://crates.io/crates/rltk/0.8.7)
* [`rltk` GitHub Repo](https://github.com/thebracket/rltk)
* [Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
* [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
* [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
* [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
* [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
* [`r/rust`](https://www.reddit.com/r/rust/)
* [`r/roguelikedev`](https://www.reddit.com/r/roguelikedev/)

### Chapter 2: Entities and Components

* [Tutorial: Writing a tiny Entity Component System (ECS) in Rust](https://ianjk.com/ecs-in-rust/)
* [Passive Data Structures](https://en.wikipedia.org/wiki/Passive_data_structure) (related to concept of POD, a.k.a. "Plain Old Data")
* [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)