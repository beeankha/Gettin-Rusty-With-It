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

### [Chapter 1: Hello Rust](https://bfnightly.bracketproductions.com/chapter_1.html)

* [`rltk` Crate Page](https://crates.io/crates/rltk/0.8.7)
* [`rltk` GitHub Repo](https://github.com/thebracket/rltk)
* [Structs](https://doc.rust-lang.org/book/ch05-00-structs.html) (Rust book chapter 5)
* [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) (Rust book chapter 10, section 2)
* [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
* [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) (Rust book chapter 4, section 2)
* [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) (Rust book chapter 3, section 1)
* [`r/rust`](https://www.reddit.com/r/rust/)
* [`r/roguelikedev`](https://www.reddit.com/r/roguelikedev/)

### [Chapter 2: Entities and Components](https://bfnightly.bracketproductions.com/chapter_2.html)

* [Tutorial: Writing a tiny Entity Component System (ECS) in Rust](https://ianjk.com/ecs-in-rust/)
* [Passive Data Structures](https://en.wikipedia.org/wiki/Passive_data_structure) (related to concept of POD, a.k.a. "Plain Old Data")
* [Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) (Rust book chapter 5, section 3)
* [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) (Rust book chapter 3, section 5)
* [Character table](http://dwarffortresswiki.org/index.php/Character_table) of [Codepage 437](https://en.wikipedia.org/wiki/Code_page_437)
* [The (Rust) Specs Book](https://specs.amethyst.rs/docs/tutorials/01_intro.html)
* [Entity component system](https://en.wikipedia.org/wiki/Entity_component_system) (Wikipedia page)
* [Tuples](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html) (from the Rust By Example book)
* [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html) (Rust book chapter 10)
* [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) (Rust book chapter 6)
* [`match`](https://doc.rust-lang.org/rust-by-example/flow_control/match.html) (from the Rust By Example book)