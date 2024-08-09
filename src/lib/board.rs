use std::{collections::{BTreeMap}, fmt::Display};

use crate::lib::Controller;

pub const BOARD_LEN: usize = 9;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum XO {
    None,
    X,
    O,
}

impl Display for XO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}",
            match self {
                Self::X => "x",
                Self::O => "o",
                Self::None => " ",
            }
        ))
    }
}

impl XO {
    pub fn select(&self) -> String {
        format!("[{}]", self)
    }
}

pub struct Board {
    pub blocks: BTreeMap<usize, XO>,
    pub position: usize,
}

impl Board {
    pub fn new() -> Self {
        let mut blocks = BTreeMap::new();
        (0..BOARD_LEN).for_each(|i| {
            blocks.insert(i as usize, XO::None);
        });

        Self {
            blocks,
            position: BOARD_LEN / 2,
        }
    }

    pub fn draw(&self) {
        Controller::clear_screen();
        println!("{}", self.make_map());
    }

    pub(crate) fn make_map(&self) -> String {
        let mut board_view = String::from("-------------\n");
        let mut current_block = 0;

        for _ in 0..(BOARD_LEN / 3) {
            let mut line = vec![String::new(); 3];

            for y in 0..(BOARD_LEN / 3) {
                if let Some(xo) = self.blocks.get(&current_block) {
                    let block: String;

                    if self.position == current_block {
                        block = xo.select();
                    } else {
                        block = format!(" {xo} ");
                    }
                    line[y] = block;
                }
                current_block += 1;
            }

            board_view.push_str(&format!(
                "|{}|{}|{}|\n-------------\n",
                line[0], line[1], line[2]
            ));
        }

        board_view
    }
}
