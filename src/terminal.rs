use std::io::stdout;

use crossterm::{ExecutableCommand, cursor};

pub fn display_cell(is_alive: bool, x: u16, y: u16) {
    let mut stdout = stdout();
    
    stdout.execute(cursor::Hide).expect("Cannot hide cursor on terminal");
    stdout.execute(cursor::MoveTo(x, y)).expect("Cannot execute terminal command");

    if is_alive {
        print!("#");
    }
    else {
        print!(" ");
    }
}