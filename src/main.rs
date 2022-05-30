use clap::Parser;
use newdoku::Sudoku;
use std::fs;
use termion::{clear, color, cursor, style};

#[derive(Parser, Debug)]
struct Config {
    /// Wait STEP millis between inserts
    #[clap(short, long, default_value_t = 0)]
    step: u64,

    /// No output until finished solving (faster)
    #[clap(short, long)]
    quiet: bool,

    /// Load Sudoku from file
    #[clap(short, long)]
    file: Option<String>,
}

fn main() {
    let config = Config::parse();

    let sudoku = match &config.file {
        Some(file) => Sudoku::from_str(&fs::read_to_string(file).unwrap()),
        _ => Sudoku::from_str(
            "xxxxxxx9xx9x7xx21xxx4x9xxxxx1xxx8xxx7xx42xxx5xx8xxxx748x1xxxx4xxxxxxxxxxxx9613xxx",
        ),
    };

    println!(
        "{}\n{}        Solving...{}",
        sudoku,
        color::Fg(color::LightRed),
        style::Reset
    );
    if let Some(sudoku) = sudoku.solution(config.step, config.quiet) {
        println!(
            "{}\n{}{}{}          Done!{}{}",
            sudoku,
            cursor::Up(14),
            clear::CurrentLine,
            color::Fg(color::LightGreen),
            style::Reset,
            cursor::Down(13)
        );
    } else {
        println!(
            "{}{}{}    No solution found{}{}{}",
            cursor::Up(1),
            clear::CurrentLine,
            color::Fg(color::LightRed),
            style::Reset,
            cursor::Down(13),
            cursor::Show
        );
    }
}
