mod grid;
use grid::Grid;
use std::{thread, time::Duration};

fn main() {
    let grid = Grid::new(200,200);
    grid.display_all();

    loop {
        const GEN_DURATION_MILISECONDS: u64 = 500; 
        thread::sleep(Duration::from_secs(GEN_DURATION_MILISECONDS));
        clear_screen();
        grid.display_all();
    }
}

fn clear_screen(){
    print!("{}[2J", 27 as char);
}

