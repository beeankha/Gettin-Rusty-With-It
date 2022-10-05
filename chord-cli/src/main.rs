use clap::Parser;
use std::collections::HashMap;

const FRETBOARD: &str = "◯ ◯ ◯ ◯ ◯ ◯
╒═╤═╤═╤═╤═╕
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
└─┴─┴─┴─┴─┘";

/// A CLI to show you how to play a guitar chord
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Name of the chord
    #[clap()]
    name: String,
}

fn main() {
    let args = Args::parse();

    let chords: HashMap<&str, &str> =
        HashMap::from([
            ("C", "x32010"),
            ("C7", "x32310"),
            ("G", "320003"),
            ("D", "xx0232"),
            ("D7", "xx0212"),
            ("Dm", "xx0231"),
            ("E", "022100"),
            ("E7", "020100"),
            ("Eb", "xx1343"),
            ("Em", "022000"),
            ("F", "133211"),
            ("G", "320003"),
            ("G7", "320001"),
            ("A", "x02220"),
            ("A7", "x02020"),
            ("Am", "002210"),
            ("B", "x24442"),
            ("B7", "x21202"),
            ("Bm", "x20432"),
            ]);

    match chords.get(&args.name[..]) {
        None => println!("Unknown chord '{}'", args.name),
        Some(pattern) => {
            let mut board: Vec<char> = FRETBOARD.chars().collect();
            for (i, ch) in pattern.chars().enumerate() {
                let idx: usize = i * 2;
                if ch == 'x' {
                    board[idx] = ch
                } else {
                    let value: usize = ch.to_digit(10).unwrap() as usize;
                    board[idx] = ' ';
                    board[idx + 24 * value] = '◯'
                }
            }
            println!(
                "\nThis is how you play the '{}' chord: \n\n{}\n",
                args.name,
                board.iter().collect::<String>()
            )
        }
    }
}
