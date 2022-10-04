## `chord-cli` Command Line Chord Helper

To install:

```
$ cargo install --path .
```

After installing, run this to get the `help` text:

```
$ chord-cli --help
```

To display a specific chord (e.g., the "C" chord):

```
$ chord-cli C                    

This is how you play the 'C' chord: 

x     ◯   ◯
╒═╤═╤═╤═╤═╕
│ │ │ │ ◯ │
├─┼─┼─┼─┼─┤
│ │ ◯ │ │ │
├─┼─┼─┼─┼─┤
│ ◯ │ │ │ │
├─┼─┼─┼─┼─┤
│ │ │ │ │ │
└─┴─┴─┴─┴─┘

```

* * * 

## References

Try out [the tutorial](https://betterprogramming.pub/build-a-command-line-tool-with-rust-to-play-guitar-chords-d07df7b330b6) for yourself!

You can heck out the tutorial author's code [here](https://github.com/yzhong52/ascii_chord).

Here is the [`clap` documentation](https://docs.rs/clap/latest/clap/), as well as information on [Rust macros](https://doc.rust-lang.org/book/ch19-06-macros.html), which make it possible for `clap` to simply annotate the `Args` and get argument parsing for free.