use crate::consoleui::ConsoleUI;

pub mod consoleui;
pub mod game;

fn main() {
    let mut console_ui = ConsoleUI::new();
    console_ui.run();
}
