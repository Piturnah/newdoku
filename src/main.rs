use std::{fmt, thread, time::Duration};
use termion::cursor;

#[derive(Debug, Clone, Copy)]
struct Sudoku {
    xs: [Option<u32>; 81],
}

impl Sudoku {
    fn from_str(src: &str) -> Self {
        let xs: [Option<u32>; 81] = src
            .chars()
            .filter(|&x| x != '\n')
            .map(|x| {
                if let Ok(num) = x.to_string().parse::<u32>() {
                    Some(num)
                } else {
                    None
                }
            })
            .collect::<Vec<Option<u32>>>()
            .try_into()
            .unwrap();
        Self { xs }
    }

    fn try_insert(&self, loc: (usize, usize), num: u32) -> Result<Self, &str> {
        assert!(loc.0 < 9, "x coord out of range in Sudoku.try_insert");
        assert!(loc.1 < 9, "y coord out of range in Sudoku.try_insert");
        assert!(num <= 9, "Inserted number must be in sudoku range (0-9)");

        let mut xs = self.xs.clone();

        for x in 0..9 {
            if xs[loc.1 * 9 + x] == Some(num) {
                return Err("Duplicate instance already in row");
            }
            if xs[x * 9 + loc.0] == Some(num) {
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
                if xs[((center.1 as isize + j) * 9 + center.0 as isize + i) as usize] == Some(num) {
                    return Err("Duplicate instance already in block");
                }
            }
        }

        xs[loc.1 * 9 + loc.0] = Some(num);
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

    fn solution(&self) -> Option<Self> {
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
                            println!("{}{}", sudoku, cursor::Up(13));
                            thread::sleep(Duration::from_millis(25));
                            if let Some(sudoku) = sudoku.solution() {
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
                                    write!(f, "{} ", num)?;
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
    let sudoku = Sudoku::from_str(
        "
.97182.34
.32...8..
.6149.257
7.834....
..4...5..
9..2.6.7.
2...6....
...8.49.5
..9....46",
    );
    println!("{}", sudoku);
    if let Some(sudoku) = sudoku.solution() {
        println!("{}", sudoku);
    } else {
        println!("No solution found");
    }
}
