use std::fmt;
use std::io;
// use std::ops::{Index, IndexMut};

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

#[derive(Clone, Copy, PartialEq)]
enum Space {
    RED,
    BLACK,
    EMPTY
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Space::RED => write!(f, "R"),
            Space::BLACK => write!(f, "B"),
            Space::EMPTY => write!(f, ".")
        }
    }
}

struct Board {
    board: [[Space; BOARD_WIDTH]; BOARD_HEIGHT]
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[Space::EMPTY; BOARD_WIDTH]; BOARD_HEIGHT]
        }
    }

    pub fn insert(&mut self, c: usize, s: Space) -> Option<(Space)> {
        let mut r = 0;
        while *self.at(r, c)? != Space::EMPTY {
            r += 1;
        }
        *self.at_mut(r, c)? = s;
        Some(Space::EMPTY)
    }

    pub fn at(&self, r: usize, c: usize) -> Option<&Space> {
        self.board.get(r)?.get(c)
    }

    pub fn at_mut(&mut self, r: usize, c: usize) -> Option<&mut Space> {
        self.board.get_mut(r)?.get_mut(c)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.board.iter().rev() {
            for sp in row.iter() {
                write!(f, "{} ", sp)?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}
fn get_col() -> Result<usize, String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .map_err(|err| err.to_string())
            .and_then(|_| {
                input.trim().parse::<usize>()
                    .map_err(|err| err.to_string())
            })
}

fn take_turn(board: &mut Board, s: Space) {
    loop {
        if let Ok(col) = get_col() {
            if let Some(victor) = board.insert(col, s) {
                match victor {
                    Space::RED => println!("Red wins!"),
                    Space::BLACK => println!("Black wins!"),
                    Space::EMPTY => break
                };
            } else {
                println!("Please select a valid column.");
            }
        } else {
            println!("Please type a nonnegative number.");
        }
    }
}

fn main() {
    let mut board = Board::new();

    loop {
        println!("Red's turn!");
        take_turn(&mut board, Space::RED);
        println!("{}", board);
        println!("Black's turn!");
        take_turn(&mut board, Space::BLACK);
        println!("{}", board);
    }
}
