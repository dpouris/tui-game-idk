use super::{Board, XO};
use console::{Key, Term};
use std::{io::{Result as IOResult}, process};


trait Distanced {
    fn is_distanced_by(&self, step: usize) -> bool;
}

impl Distanced for Vec<&usize> {
    fn is_distanced_by(&self, step: usize) -> bool {
        self.iter().skip(1).fold::<Vec<&usize>, _>(vec![*self.first().unwrap()], |mut acc, curr_elem| {
            if **acc.last().unwrap() + step == **curr_elem {
                acc.push(*curr_elem);
            }
            acc
        }).len() == self.len()
    }
}

pub struct Controller {
    pub board: Board,
    pub player: XO,
    pub(crate) term: Term,
}

impl Controller {
    #![allow(unused)]

    pub fn new() -> Self {
        let term = Term::stdout();
        term.hide_cursor();

        Self {
            board: Board::new(),
            player: XO::X,
            term,
        }
    }

    pub fn exit(&self) -> ! {
        println!("Exiting...");
        self.term.show_cursor();
        process::exit(0)
    }

    pub fn clear_screen() {
        print!("\x1b[2J\x1b[2H");
    }

    pub fn clear_line() {
        print!("\x1b[1A\x1b[2K");
    }

    pub fn get_input(&mut self) -> IOResult<Key> {
        self.term.read_key()
    }

    pub fn handle_input(&mut self, key: Key) -> Option<XO> {
        match key {
            Key::Char('q') => self.exit(),
            Key::Enter => {
                if let Some(XO::None) = self.board.blocks.get(&self.board.position) {
                    self.board
                    .blocks
                    .insert(self.board.position, self.player);

                if let Some(who_won) = self.check_win() {
                    return Some(who_won)
                }
                if self.check_draw() {
                    return Some(XO::None);
                }
                    self.switch_turn();
                }
            }
            x => self.update_cursor_position(x),
        };
        None
    }

    pub fn player(&mut self, xo: XO) {
        self.player = xo;
    }

    fn switch_turn(&mut self) {
        self.player(if self.player == XO::X {XO::O} else {XO::X});
    }

    fn check_win(&self) -> Option<XO> {
        let mut active_player_blocks = self.get_block_indices::<3>();
        let is_horizontal = active_player_blocks.iter().filter(|v| v.iter().all(|v| **v == self.player)).collect::<Vec<&[&XO;3]>>().len() > 0;
        let is_vertical = !(0..3).filter(|i| active_player_blocks.iter().all(|v| *v[*i as usize] == self.player)).collect::<Vec<i32>>().is_empty();
        let is_diagonal = active_player_blocks.iter().enumerate().map(|(idx, v)| *v[idx] == self.player).all(|b| b);
        let is_rev_diagonal = active_player_blocks.iter().rev().enumerate().map(|(idx, v)| *v[idx] == self.player).all(|b| b);

        if is_horizontal || is_vertical || is_diagonal || is_rev_diagonal {
            return Some(self.player);
        }

        None
    }

    fn check_draw(&self) -> bool {
        self.board.blocks.iter().all(|(_, v)| *v != XO::None) 
    }

    fn get_block_indices<const U: usize>(&self) -> Vec<[&XO;U]> {
        self.board.blocks.values().array_chunks::<U>().fold(vec![], |mut acc, v| {
            acc.push(v);
            acc
        })
    }

    
    fn update_cursor_position(&mut self, key: Key) {
        match key {
            Key::ArrowUp => {
                if self.board.position < 3 {
                    self.board.position = 0;
                } else {
                    self.board.position -= 3;
                }
            }
            Key::ArrowDown => {
                if self.board.position + 3 > 8 {
                    self.board.position = 8;
                } else {
                    self.board.position += 3
                }
            }
            Key::ArrowLeft => {
                if self.board.position > 0 {
                    self.board.position -= 1
                }
            }
            Key::ArrowRight => {
                if self.board.position < 8 {
                    self.board.position += 1;
                }
            }

            _ => {}
        };
    }
}
