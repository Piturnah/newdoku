# Newdoku | [![](https://img.shields.io/crates/v/newdoku)](https://crates.io/crates/newdoku)

![](https://github.com/Piturnah/newdoku/blob/master/demo.gif)

Simple Sudoku solver written in Rust. Method is inspired by a method briefly outlined in a CS lecture I had that I decided to try and implement based only on my understanding from the lecture.

It is called Newdoku because I started doing this a while ago and forgot about it and decided to restart, hence "New"

## Quick Start

Currently works only in ANSI terminals.

```console
$ cargo run
```

```console
OPTIONS:
    -f, --file <FILE>    Load Sudoku from file
    -h, --help           Print help information
    -q, --quiet          No output until finished solving (faster)
    -s, --step <STEP>    Wait STEP millis between inserts [default: 0]
```

When loading from a file, the parser parses any digit as a number in the sudoku. Any other character other than a newline will be parsed as an empty square.

Example sudoku:
`xxxxxxx9xx9x7xx21xxx4x9xxxxx1xxx8xxx7xx42xxx5xx8xxxx748x1xxxx4xxxxxxxxxxxx9613xxx`

Gives
```console
+-------+-------+-------+
| . . . | . . . | . 9 . |
| . 9 . | 7 . . | 2 1 . |
| . . 4 | . 9 . | . . . |
+-------+-------+-------+
| . 1 . | . . 8 | . . . |
| 7 . . | 4 2 . | . . 5 |
| . . 8 | . . . | . 7 4 |
+-------+-------+-------+
| 8 . 1 | . . . | . 4 . |
| . . . | . . . | . . . |
| . . 9 | 6 1 3 | . . . |
+-------+-------+-------+
```

## Contributing

PRs and issues welcome.
