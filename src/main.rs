use clap::Parser;
use std::{fmt, fs, thread, time::Duration};
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum SudokuNum {
    Original(u32),
    Edited(u32),
}

impl fmt::Display for SudokuNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Original(num) => write!(f, "{}", num),
            Self::Edited(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Sudoku {
    xs: [Option<SudokuNum>; 81],
}

impl Sudoku {
    fn from_str(src: &str) -> Self {
        use SudokuNum::*;
        let xs: [Option<SudokuNum>; 81] = src
            .chars()
            .filter(|&x| x != '\n')
            .map(|x| {
                if let Ok(num) = x.to_string().parse::<u32>() {
                    Some(Original(num))
                } else {
                    None
                }
            })
            .collect::<Vec<Option<SudokuNum>>>()
            .try_into()
            .unwrap();
        Self { xs }
    }

    fn try_insert(&self, loc: (usize, usize), num: u32) -> Result<Self, &str> {
        use SudokuNum::*;
        assert!(loc.0 < 9, "x coord out of range in Sudoku.try_insert");
        assert!(loc.1 < 9, "y coord out of range in Sudoku.try_insert");
        assert!(num <= 9, "Inserted number must be in sudoku range (0-9)");

        let mut xs = self.xs.clone();

        for x in 0..9 {
            if (xs[loc.1 * 9 + x] == Some(Original(num))) | (xs[loc.1 * 9 + x] == Some(Edited(num)))
            {
                return Err("Duplicate instance already in row");
            }
            if (xs[x * 9 + loc.0] == Some(Original(num))) | (xs[x * 9 + loc.0] == Some(Edited(num)))
            {
                return Err("Duplicate instance already in col");
            }
        }

        let rel_center = |origin: usize| match origin % 3 {
            0 => origin + 1,
            1 => origin,
            2 => origin - 1,
            _ => panic!("Unreachable"),
        };

        let center = (rel_center(loc.0), rel_center(loc.1));
        for i in -1..2 {
            for j in -1..2 {
                let x = xs[((center.1 as isize + j) * 9 + center.0 as isize + i) as usize];
                if (x == Some(Original(num))) | (x == Some(Edited(num))) {
                    return Err("Duplicate instance already in block");
                }
            }
        }

        xs[loc.1 * 9 + loc.0] = Some(Edited(num));
        Ok(Self { xs })
    }

    fn is_full(&self) -> bool {
        for x in self.xs {
            if x.is_none() {
                return false;
            }
        }
        true
    }

    fn solution(&self, config: &Config) -> Option<Self> {
        print!("{}", cursor::Hide);

        if self.is_full() {
            print!("{}", cursor::Show);
            return Some(self.clone());
        }

        for i in 0..9 {
            for j in 0..9 {
                if self.xs[i * 9 + j].is_none() {
                    for x in 1..10 {
                        if let Ok(sudoku) = self.try_insert((j, i), x) {
                            if !config.quiet {
                                println!("{}{}", sudoku, cursor::Up(13));
                            }
                            if config.step > 0 {
                                thread::sleep(Duration::from_millis(config.step));
                            }

                            if let Some(sudoku) = sudoku.solution(&config) {
                                return Some(sudoku);
                            }
                        }
                    }
                    return None;
                }
            }
        }

        print!("{}", cursor::Show);
        None
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use SudokuNum::*;
        let mut xs = self.xs.iter();
        for row in 0..13 {
            match row {
                0 | 4 | 8 => {
                    writeln!(f, "+-------+-------+-------+")?;
                }
                12 => {
                    write!(f, "+-------+-------+-------+")?;
                }
                _ => {
                    write!(f, "| ")?;
                    for x in 0..11 {
                        match x {
                            3 | 7 => {
                                write!(f, "| ")?;
                            }
                            _ => {
                                if let Some(num) = xs.next().unwrap() {
                                    match num {
                                        Original(num) => {
                                            write!(f, "{}{}{} ", style::Bold, num, style::Reset)?
                                        }
                                        Edited(num) => write!(f, "{} ", num)?,
                                    }
                                } else {
                                    write!(f, ". ")?;
                                }
                            }
                        }
                    }
                    writeln!(f, "|")?;
                }
            }
        }
        Ok(())
    }
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
    if let Some(sudoku) = sudoku.solution(&config) {
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
