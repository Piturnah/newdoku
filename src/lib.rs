//! Utilities for displaying Sudokus and solving them in ANSI-compliant terminals.
//!
//! # Quick Start
//!
//! Print the solution to a Sudoku in terminal:
//!
//! ```
//! use newdoku::Sudoku;
//!
//! let s = Sudoku::from_str(
//!     "xxxxxxx9xx9x7xx21xxx4x9xxxxx1xxx8xxx7xx42xxx5xx8xxxx748x1xxxx4xxxxxxxxxxxx9613xxx",
//! );
//!
//! println!("{}\n\n{}", s, s.solution(0, false).unwrap());
//! ```

use crossterm::{cursor, style::Attribute};
use std::{fmt, thread, time::Duration};

#[derive(Debug, Clone, Copy)]
enum SudokuNum {
    Original(u8),
    Edited(u8),
}

impl PartialEq for SudokuNum {
    fn eq(&self, rhs: &Self) -> bool {
        use SudokuNum::*;
        let x = match self {
            Original(x) => x,
            Edited(x) => x,
        };
        let y = match rhs {
            Original(y) => y,
            Edited(y) => y,
        };

        x == y
    }
}

impl fmt::Display for SudokuNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Original(num) => write!(f, "{}", num),
            Self::Edited(num) => write!(f, "{}", num),
        }
    }
}

/// Contains an 81-size array of [`Option<u8>`].
#[derive(Debug, Clone, Copy)]
pub struct Sudoku {
    xs: [Option<SudokuNum>; 81],
}

impl Sudoku {
    /// Returns a [`Sudoku`] from a given `src: &str`. Digits are parsed as a number in the sudoku while anything else is a blank space. Newlines are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use newdoku::Sudoku;
    ///
    /// Sudoku::from_str(
    ///     "xxxxxxx9xx9x7xx21xxx4x9xxxxx1xxx8xxx7xx42xxx5xx8xxxx748x1xxxx4xxxxxxxxxxxx9613xxx"
    /// );
    /// ```
    pub fn from_str(src: &str) -> Self {
        use SudokuNum::*;
        let xs: [Option<SudokuNum>; 81] = src
            .chars()
            .filter(|&x| x != '\n')
            .map(|x| {
                if let Ok(num) = x.to_string().parse::<u8>() {
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

    /// Returns a [`Sudoku`] that is the same as `self` but with `num` inserted at `loc: (x, y)` (0-indexed) if it can be inserted there by sudoku rules.
    pub fn try_insert(&self, loc: (usize, usize), num: u8) -> Result<Self, &str> {
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

        let rel_center = |origin| origin + 1 - origin % 3;
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

    /// Returns true if `self` has no empty spaces.
    pub fn is_full(&self) -> bool {
        for x in self.xs {
            if x.is_none() {
                return false;
            }
        }
        true
    }

    /// Returns the solved [`Sudoku`] if it exists. If `quiet` set to false, then prints each iteration while solving.
    ///
    /// # Examples
    ///
    /// ```
    /// use newdoku::Sudoku;
    ///
    /// let s = Sudoku::from_str(
    ///     "xxxxxxx9xx9x7xx21xxx4x9xxxxx1xxx8xxx7xx42xxx5xx8xxxx748x1xxxx4xxxxxxxxxxxx9613xxx",
    /// );
    /// assert_eq!(
    ///     s.solution(0, true).unwrap(),
    ///     Sudoku::from_str(
    ///         "157832496396745218284196753415378962763429185928561374831257649672984531549613827"
    ///     )
    /// );
    /// ```
    pub fn solution(&self, step: u64, quiet: bool) -> Option<Self> {
        print!("{}", cursor::Hide);

        if self.is_full() {
            print!("{}", cursor::Show);
            return Some(*self);
        }

        for i in 0..9 {
            for j in 0..9 {
                if self.xs[i * 9 + j].is_none() {
                    for x in 1..10 {
                        if let Ok(sudoku) = self.try_insert((j, i), x) {
                            if !quiet {
                                println!("{}\n\n{}", sudoku, cursor::MoveUp(15));
                            }
                            if step > 0 {
                                thread::sleep(Duration::from_millis(step));
                            }

                            if let Some(sudoku) = sudoku.solution(step, quiet) {
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

impl PartialEq for Sudoku {
    fn eq(&self, rhs: &Self) -> bool {
        let mut rhs = rhs.xs.into_iter();
        for x in self.xs {
            if x != rhs.next().unwrap() {
                return false;
            }
        }
        true
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
                                        Original(num) => write!(
                                            f,
                                            "{}{}{} ",
                                            Attribute::Bold,
                                            num,
                                            Attribute::Reset
                                        )?,
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

#[cfg(test)]
mod test {
    use super::*;
    const TEST_SUDOKU: &str =
        "xxxxxxx9xx9x7xx21xxx4x9xxxxx1xxx8xxx7xx42xxx5xx8xxxx748x1xxxx4xxxxxxxxxxxx9613xxx";
    const SOLVED_SUDOKU: &str =
        "157832496396745218284196753415378962763429185928561374831257649672984531549613827";

    #[test]
    fn par_eq_sudokunum() {
        assert_eq!(SudokuNum::Original(5), SudokuNum::Edited(5));
    }

    #[test]
    fn par_eq_sudoku() {
        let s1 = Sudoku::from_str(TEST_SUDOKU);
        let s2 = Sudoku::from_str(TEST_SUDOKU);

        assert_eq!(s1, s2);
    }

    #[test]
    fn solve() {
        let s = Sudoku::from_str(TEST_SUDOKU);
        assert_eq!(
            s.solution(0, true).unwrap(),
            Sudoku::from_str(SOLVED_SUDOKU)
        );
    }

    #[test]
    #[should_panic]
    fn solve_invalid() {
        let s = Sudoku::from_str(
            "xxxxxxx9xx9x7xx21xxx4x9xxxxx1xxx8xxx7xx42xxx5xx8xxxx748x1xxxx4xxxxxxxxxxxx9613xx5",
        );
        s.solution(0, true).unwrap();
    }

    #[test]
    fn try_insert() {
        let s1 = Sudoku::from_str(TEST_SUDOKU).try_insert((3, 2), 5).unwrap();
        let s2 = Sudoku::from_str(
            "xxxxxxx9xx9x7xx21xxx459xxxxx1xxx8xxx7xx42xxx5xx8xxxx748x1xxxx4xxxxxxxxxxxx9613xxx",
        );
        assert_eq!(s1, s2);
    }

    #[test]
    fn try_insert_row() {
        let s1 = Sudoku::from_str(TEST_SUDOKU);
        assert_eq!(
            s1.try_insert((5, 6), 4),
            Err("Duplicate instance already in row")
        );
    }

    #[test]
    fn try_insert_col() {
        let s1 = Sudoku::from_str(TEST_SUDOKU);
        assert_eq!(
            s1.try_insert((5, 7), 8),
            Err("Duplicate instance already in col")
        );
    }

    #[test]
    fn try_insert_block() {
        let s1 = Sudoku::from_str(TEST_SUDOKU);
        assert_eq!(
            s1.try_insert((5, 6), 6),
            Err("Duplicate instance already in block")
        );
    }

    #[test]
    fn is_full() {
        assert_eq!(Sudoku::from_str(SOLVED_SUDOKU).is_full(), true);
    }

    #[test]
    fn isnt_full() {
        assert_eq!(Sudoku::from_str(TEST_SUDOKU).is_full(), false);
    }
}
