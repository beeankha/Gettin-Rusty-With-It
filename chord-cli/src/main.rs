const FRETBOARD: &str = "◯ ◯ ◯ ◯ ◯ ◯
┌─┬─┬─┬─┬─┐
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
└─┴─┴─┴─┴─┘";

use clap::Parser;

/// A CLI tool that displays guitar chords
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Name of the chord
    #[clap()]
    name: String,
}

fn main() {
    let args = Args::parse();

    println!("\nThis is how you play the '{}' chord: \n\n{}\n", args.name, FRETBOARD);
}
