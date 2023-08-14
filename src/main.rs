mod grid;
use grid::Grid;
use std::{thread, time::Duration};
use std::io;

fn main() {
    let mut grid = Grid::new(100,30);
    grid.display_all();

    loop {
        const GEN_DURATION_MILISECONDS: u64 = 200; 
        thread::sleep(Duration::from_millis(GEN_DURATION_MILISECONDS));
        grid.calculate_cell_life();
        clear_screen();
        grid.display_all();
    }
}

fn clear_screen(){
    print!("\x1B[2J\x1B[1;1H");
}

