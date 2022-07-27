use crate::game::*;
use colored::Colorize;
use std::io;

pub struct ConsoleUI {
    field: PlayField,
}

impl ConsoleUI {
    fn print_help() {
        println!("\nAvailable Commands:");
        println!("    O x y    - open cell");
        println!("    F x y    - flag cell (toggle)");
        println!("    D        - draw board");
        println!("    Q        - quit\n");
    }

    fn parse_position(p: Vec<&str>) -> Result<(usize, usize), String> {
        if p.len() != 2 {
            return Err(format!(
                "Wrong parameter count. Expected 2, received {}.",
                p.len()
            ));
        }

        let parse_x = p[0].parse::<usize>();
        let x = match parse_x {
            Ok(parse_x) => parse_x,
            Err(e) => return Err(e.to_string()),
        };

        let parse_y = p[1].parse::<usize>();
        let y = match parse_y {
            Ok(parse_y) => parse_y,
            Err(e) => return Err(e.to_string()),
        };

        Ok((x, y))
    }

    fn printboard(&mut self) {
        println!();
        let board = self.field.get_playfield();
        let mut c = 0;
        let mut line = 1;
        println!("     1  2  3  4  5  6  7  8  9 10");
        println!("   ------------------------------");
        print!("{: >2}| ", line);
        for field in board {
            print!(
                " {} ",
                match field {
                    PlayFieldSpot::Closed => "#".clear(),
                    PlayFieldSpot::Flagged => "!".white().on_blue(),
                    PlayFieldSpot::Bomb => "✱".red(),
                    PlayFieldSpot::Open(v) => {
                        match v {
                            1 => "1".blue(),
                            2 => "2".green(),
                            3 => "3".yellow(),
                            4 => "4".red(),
                            5 => "5".bold().blue(),
                            6 => "6".bold().green(),
                            7 => "7".bold().yellow(),
                            8 => "8".bold().red(),
                            _ => " ".clear(),
                        }
                    }
                }
            );
            c += 1;
            if c == self.field.get_width() {
                c = 0;
                println!();
                line += 1;
                if line <= self.field.get_height() {
                    print!("{: >2}| ", line);
                }
            }
        }
    }

    pub fn new() -> ConsoleUI {
        let f = PlayField::new(10, 10, 10);
        ConsoleUI { field: f }
    }

    pub fn run(&mut self) {
        loop {
            self.printboard();
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Err(error) => {
                    println!("Read line failed: {}", error);
                    break;
                }
                Ok(_) => {}
            }

            let line = buffer.trim();
            if line == "" {
                continue;
            }
            let mut iter = buffer.trim().split(" ");
            let cmd = match iter.next() {
                None => continue,
                Some(val) => val,
            };
            let params: Vec<&str> = iter.collect();

            let res = match cmd {
                "Q" | "q" => break,
                "P" | "p" => {
                    self.printboard();
                    continue;
                }
                "F" | "f" => {
                    let (x, y) = match ConsoleUI::parse_position(params) {
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                        Ok(v) => v,
                    };
                    match self.field.toggle_flag(x, y) {
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                        Ok(s) => s,
                    }
                }
                "O" | "o" => {
                    let (x, y) = match ConsoleUI::parse_position(params) {
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                        Ok(v) => v,
                    };
                    match self.field.open(x, y) {
                        Err(e) => {
                            println!("{}", e);
                            continue;
                        }
                        Ok(s) => s,
                    }
                }
                _ => {
                    ConsoleUI::print_help();
                    continue;
                }
            };

            match res {
                GameState::KeepPlaying => continue,
                GameState::YouWin => {
                    self.printboard();
                    println!("You win");
                }
                GameState::YouLose => {
                    self.printboard();
                    println!("You lose");
                }
            }
            break;
        }
    }
}
