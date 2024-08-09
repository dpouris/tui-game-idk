use super::{Controller, XO};
use console::Key;
use std::io::{Error, Result as IOResult};

pub struct Game {
    controller: Controller,
}

impl Game {
    pub fn new() -> Self {
        Self {
            controller: Controller::new(),
        }
    }

    pub fn start(&mut self) -> Result<XO, Error> {
        self.set_active_player()?;
        
        let who_won = loop {
            self.controller.board.draw();
            println!("{} is playing!", self.controller.player);
            let inp = self.controller.get_input()?;
            if let Some(who_won) = self.controller.handle_input(inp) {
                self.controller.board.draw();
                break who_won
            };
        };

        Ok(who_won)
    }

    pub fn restart(mut self) -> Option<Self> {
        let mut selected = "Yeah!";
        let mut can_go_left = false;
        let select = |option: &'static str, selected: &str| {
            if option == selected {
                return format!("[{option}]");
            }
            format!(" {option} ")
        };
        
        loop {
            println!("Restart? |{} or {}|", select("Yeah!", selected), select("Quit", selected));
            let key = self.controller.get_input().expect("could not parse user input");
            match key {
                Key::ArrowLeft => {
                    if can_go_left {
                        selected = "Yeah!";
                        can_go_left = false;
                    }
                }
                Key::ArrowRight => {
                    if !can_go_left {
                        selected = "Quit";
                        can_go_left = true;
                    }
                }
                Key::Enter => break,
                Key::Char('q') => self.controller.exit(),
                _ => {}
            };
            Controller::clear_line();
        }

        if selected == "Yeah!" {
            return Some(Game::new());
        }

        self.controller.exit();
    }

    fn set_active_player(&mut self) -> IOResult<()> {
        let mut can_go_left = false;
        let mut symbol = XO::X;

        let select_if_same = |xo: XO, symbol: &XO| {
            if *symbol == xo {
                return xo.select();
            }
            format!(" {} ", xo)
        };

        loop {
            Controller::clear_screen();
            println!(
                "Choose symbol:\n|{} or {}|",
                select_if_same(XO::X, &symbol),
                select_if_same(XO::O, &symbol)
            );
            let key = self.controller.get_input()?;
            match key {
                Key::ArrowLeft => {
                    if can_go_left {
                        symbol = XO::X;
                        can_go_left = false;
                    }
                }
                Key::ArrowRight => {
                    if !can_go_left {
                        symbol = XO::O;
                        can_go_left = true;
                    }
                }
                Key::Enter => break,
                Key::Char('q') => self.controller.exit(),
                _ => {}
            };
        }

        self.controller.player(symbol);
        
        Ok(())
    }
}
