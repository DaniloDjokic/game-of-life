mod grid;
mod cell;
use std::io;

use grid::Grid;

fn main() {
    let grid = Grid::new(100, 100);
    grid.display_all();

    let mut s = String::from("value");

    io::stdin().read_line(&mut s).expect("oof");
}

