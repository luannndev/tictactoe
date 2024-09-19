extern crate rand;
use self::rand::Rng;

use std::fmt::*;
use std::clone::*;

#[derive(PartialEq)]
#[derive(Clone)]
pub enum BoardState {
    X, O, Empty
}

impl BoardState {
    fn to_string(&self) -> String {
        match *self {
            BoardState::X => "X".to_string(),
            BoardState::O => "O".to_string(),
            BoardState::Empty => " ".to_string()
        }
    }
}

impl Debug for BoardState {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

pub enum GameResult {
    XWon, OWon, Tie, InProgress
}


pub struct Game {
    board: [BoardState; 9],
    winner: GameResult
}

impl Game {
    pub fn new() -> Game {
        Game { board: [BoardState::Empty, BoardState::Empty, BoardState::Empty,
            BoardState::Empty, BoardState::Empty, BoardState::Empty,
            BoardState::Empty, BoardState::Empty, BoardState::Empty],
            winner: GameResult::InProgress }
    }

    pub fn get_board(&self) -> String {
        let mut result = "  A B C \n".to_string();
        result.push_str(" ┌─┬─┬─┐\n");

        let mut i = 0;
        while i < 8 {
            let fmt = format!("{}│{:?}│{:?}│{:?}│\n", (i / 3)+1, self.board[i],
                              self.board[i+1],
                              self.board[i+2]);
            result.push_str(&fmt);

            if i < 6 {
                result.push_str(" ├─┼─┼─┤\n");
            }
            else {
                result.push_str(" └─┴─┴─┘\n");
            }
            i += 3;
        }
        result
    }

    fn parse_location_col(c: Option<char>) -> Option<i32> {
        match c {
            Some('A') => Some(0),
            Some('B') => Some(1),
            Some('C') => Some(2),
            _         => None
        }
    }

    fn parse_location_row(c: Option<char>) -> Option<i32> {
        match c {
            Some('1') => Some(0),
            Some('2') => Some(3),
            Some('3') => Some(6),
            _         => None
        }
    }

    pub fn parse_location(location: &String) -> Option<i32> {
        let mut chars = location.chars();
        let col = Game::parse_location_col(chars.next());
        let row = Game::parse_location_row(chars.next());
        col.and_then(|x| row.map(|y| x+y))
    }

    pub fn mark(&mut self, location: i32, state: BoardState) {
        self.board[location as usize] = state;
    }

    fn set_winner(&mut self, winner: &BoardState) {
        self.winner =
            match *winner {
                BoardState::X => GameResult::XWon,
                BoardState::O => GameResult::OWon,
                BoardState::Empty => GameResult::InProgress
            };
    }

    pub fn check(&mut self) -> bool {
        let mut i = 0;
        while i < 9 {
            let first = self.board[i].clone();
            if first == BoardState::Empty {
                break;
            }
            if first == self.board[i+1] && first == self.board[i+2] {
                self.set_winner(&first);
                return true;
            }
            i += 3;
        }

        i = 0;
        while i < 9 {
            let first = self.board[i].clone();
            if first == BoardState::Empty {
                break;
            }
            if first == self.board[i+3] && first == self.board[i+6] {
                self.set_winner(&first);
                return true;
            }
            i += 3;
        }

        if self.board[0] == self.board[4] && self.board[0] == self.board[8] &&
            self.board[0] != BoardState::Empty {
            let x = self.board[0].clone();
            self.set_winner(&x);
            return true;
        }
        if self.board[2] == self.board[4] && self.board[2] == self.board[6] &&
            self.board[2] != BoardState::Empty {
            let x = self.board[2].clone();
            self.set_winner(&x);
            return true;
        }

        for i in 0 .. 8 {
            if self.board[i] == BoardState::Empty {
                return false;
            }
        }
        self.winner = GameResult::Tie;
        true
    }

    pub fn take_turn(&mut self) {
        loop {
            let index = rand::thread_rng().gen_range(0..8);
            if self.board[index] == BoardState::Empty {
                self.mark(index as i32, BoardState::O);
                break;
            }
        }
    }

    pub fn get_winner(&mut self) -> &'static str {
        match self.winner {
            GameResult::XWon => "X",
            GameResult::OWon => "O",
            _ => "Nobody"
        }
    }
}