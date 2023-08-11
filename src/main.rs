mod grid;
use grid::Grid;

fn main() {
    let grid = Grid::new(100, 100);
    grid.display_all();
}

