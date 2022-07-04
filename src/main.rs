use crossterm::{
    cursor,
    style::{Attribute, Color, SetForegroundColor},
    terminal::{Clear, ClearType::CurrentLine},
};
use newdoku::{clap::Parser, Sudoku};
use std::fs;

#[derive(Parser, Debug)]
struct Config {
    /// Wait STEP millis between inserts
    #[clap(short, long, default_value_t = 0)]
    step: u64,

    /// No output until finished solving (faster)
    #[clap(short, long)]
    quiet: bool,

    /// Load Sudoku by unique ID
    #[clap(short, long)]
    uid: Option<String>,

    /// Load Sudoku from file
    #[clap(short, long)]
    file: Option<String>,
}

fn main() {
    let config = Config::parse();

    let sudoku = match &config.file {
        Some(file) => Sudoku::from_str(&fs::read_to_string(file).unwrap()),
        _ => match &config.uid {
            Some(uid) => Sudoku::from_str(uid),
            _ => Sudoku::from_str(
                "xxxxxxx9xx9x7xx21xxx4x9xxxxx1xxx8xxx7xx42xxx5xx8xxxx748x1xxxx4xxxxxxxxxxxx9613xxx",
            ),
        },
    };

    println!(
        "{}\n{}        Solving...{}",
        sudoku,
        SetForegroundColor(Color::Red),
        Attribute::Reset
    );
    if let Some(sudoku) = sudoku.solution(config.step, config.quiet) {
        println!(
            "{}\n{}{}{}          Done!{}{}",
            sudoku,
            cursor::MoveUp(14),
            Clear(CurrentLine),
            SetForegroundColor(Color::Green),
            Attribute::Reset,
            cursor::MoveDown(13)
        );
    } else {
        println!(
            "{}{}{}    No solution found{}{}{}",
            cursor::MoveUp(1),
            Clear(CurrentLine),
            SetForegroundColor(Color::Red),
            Attribute::Reset,
            cursor::MoveDown(13),
            cursor::Show
        );
    }
}
