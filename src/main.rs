use std::fmt;
use std::io;
// use std::ops::{Index, IndexMut};

const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;
const LENGTH_TO_WIN: i32 = 4;

#[derive(Clone, Copy, PartialEq)]
enum Space {
    RED,
    BLACK,
    EMPTY,
}

impl Space {
    fn opposing(self) -> Space {
        // passing 'self' by value suggested by Clippy. Does it actually make sense?
        match self {
            Space::RED => Space::BLACK,
            Space::BLACK => Space::RED,
            Space::EMPTY => panic!("Called opposing with empty!"),
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Space::RED => write!(f, "R"),
            Space::BLACK => write!(f, "B"),
            Space::EMPTY => write!(f, "."),
        }
    }
}

#[derive(Clone, Copy)]
struct Board {
    board: [[Space; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[Space::EMPTY; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    fn win_in_direction(
        &mut self,
        (start_r, start_c): (i32, i32),
        (dir_r, dir_c): (i32, i32),
        start_s: Space,
    ) -> bool {
        let mut count_forward = 0;
        while let Some(s) = self.get(
            start_r + (dir_r * count_forward),
            start_c + (dir_c * count_forward),
        ) {
            if start_s == *s {
                count_forward += 1;
            } else {
                break;
            }
        }
        let mut count_backward = 0;
        while let Some(s) = self.get(
            start_r - dir_r * (count_backward + 1),
            start_c - dir_c * (count_backward + 1),
        ) {
            if start_s == *s {
                count_backward += 1;
            } else {
                break;
            }
        }
        count_forward + count_backward >= LENGTH_TO_WIN
    }

    pub fn insert(&mut self, c: i32, s: Space) -> Option<bool> {
        let mut r = 0;
        while *self.get(r, c)? != Space::EMPTY {
            r += 1;
        }
        *self.get_mut(r, c)? = s;
        let directions: [(i32, i32); 4] = [(0, 1), (1, 1), (1, 0), (1, -1)];
        if directions
            .iter()
            .any(|&x| self.win_in_direction((r, c), x, s))
        {
            // win, so return true
            Some(true)
        } else {
            // don't win, so return false
            Some(false)
        }
    }

    pub fn get(&self, r: i32, c: i32) -> Option<&Space> {
        let r = r as usize;
        let c = c as usize;
        self.board.get(r)?.get(c)
    }

    // TODO: is there a better way of having get and get_mut such that they aren't copy/pasted?
    pub fn get_mut(&mut self, r: i32, c: i32) -> Option<&mut Space> {
        let r = r as usize;
        let c = c as usize;
        self.board.get_mut(r)?.get_mut(c)
    }

    pub fn get_available_columns(&self) -> Vec<i32> {
        let mut available: Vec<i32> = Vec::new();
        let mut index = 0;
        while let Some(s) = self.get((BOARD_HEIGHT - 1) as i32, index) {
            if *s == Space::EMPTY {
                available.push(index);
            }
            index += 1;
        }
        available
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..BOARD_WIDTH {
            print!("{} ", i,);
        }
        println!();
        for row in self.board.iter().rev() {
            for sp in row.iter() {
                write!(f, "{} ", sp)?;
            }
            writeln!(f)?;
        }
        write!(f, "") // TODO is there a better way?
    }
}

fn get_col() -> Result<i32, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|err| err.to_string())
        .and_then(|_| input.trim().parse::<i32>().map_err(|err| err.to_string()))
}

fn take_turn_human(board: &mut Board, s: Space) -> bool {
    loop {
        if let Ok(col) = get_col() {
            if let Some(game_ended) = board.insert(col, s) {
                if game_ended {
                    println!("{} wins!", s);
                    return true;
                } else {
                    break;
                }
            } else {
                println!("Please select a valid column.");
            }
        } else {
            println!("Please type a valid number.");
        }
    }
    false
}

fn minimax(board: &Board, player: Space) -> i32 {
    let mut board_stack = Vec::new();

    // search starts with the board as it is
    board_stack.push(board.clone());

    let (choice, _) = _minimax(board, player, 0);
    choice
}

const MAX_DEPTH: i32 = 6;

fn _minimax(board: &Board, player: Space, depth: i32) -> (i32, i32) {
    let available_cols = board.get_available_columns();
    let mut best_choice = match available_cols.first() {
        Some(x) => *x,
        None => return (0, 0),
    };
    let mut best_score = i32::min_value();

    if depth > MAX_DEPTH {
        return (available_cols[0], 0);
    }

    for col in available_cols {
        let mut local_board = board.clone();
        let won = local_board.insert(col, player).unwrap();
        if won {
            return (col, 1);
        }
        let (_, score) = _minimax(&local_board, player.opposing(), depth + 1);
        if -score > best_score {
            best_score = -score;
            best_choice = col;
        }
    }
    (best_choice, best_score)
}

fn main() {
    let mut board = Board::new();

    let mut current_player = Space::RED;
    println!("{}", board);
    loop {
        let game_is_over = match current_player {
            Space::RED => {
                println!("Red's turn!");
                take_turn_human(&mut board, current_player)
            }
            Space::BLACK => {
                println!("Black's turn!");
                board
                    .insert(minimax(&board, Space::BLACK), Space::BLACK)
                    .unwrap()
            }
            Space::EMPTY => panic!("This should never happen!"),
        };
        println!("{}", board);

        if game_is_over {
            break;
        }

        current_player = current_player.opposing();
    }
    println!("{}", board);
    println!("Game over, {} wins! Thanks for playing!", current_player)
}
