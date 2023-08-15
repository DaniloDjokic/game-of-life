extern crate crossterm;

mod grid;
mod terminal;
use grid::Grid;
use std::{thread, time::Duration};

fn main() {
    let mut grid = Grid::new(100,40, 0.1);
    grid.display_all();

    loop {
        const GEN_DURATION_MILISECONDS: u64 = 250; 
        thread::sleep(Duration::from_millis(GEN_DURATION_MILISECONDS));
        grid.calculate_cell_life();
        grid.display_all_and_update();
    }
}
