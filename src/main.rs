#![feature(iter_array_chunks)]
#![feature(slice_flatten)]

use std::io::Error;
use lib::Game;

use crate::lib::XO;

mod lib;

fn main() -> Result<(), Error> {
    let mut game = Game::new();
    
    loop {
        let who_won = game.start()?;

        match who_won {
            XO::None => println!("It's a draw!\n"),
            _ => println!("Player {} won!\n", who_won)
        }

        if let Some(g) = game.restart() {
            game = g;
            continue;
        }
        println!("\x1b[2J\x1b[2HGoodbye...");
        break
    }

    Ok(())
}
