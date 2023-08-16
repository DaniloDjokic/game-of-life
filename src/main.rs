extern crate crossterm;

mod grid;
mod terminal;
mod config;

use grid::Grid;
use std::{thread, time::Duration};
use config::Config;

fn main(){
    let config = Config::init_from_file();
    let mut grid = init_grid(&config);
    play_game(&mut grid);
}

fn init_grid(config: &Config) -> Grid {
    let grid = Grid::new(config.width, config.height, &config.seed, config.initial_field_size);
    grid.display_all();
    grid
}

fn play_game(grid: &mut Grid){
    grid.calculate_cell_life();
    grid.display_all_and_update();

    loop {
        const GEN_DURATION_MILISECONDS: u64 = 250; 
        thread::sleep(Duration::from_millis(GEN_DURATION_MILISECONDS));
        grid.calculate_cell_life();
        grid.display_all_and_update();
    }
}