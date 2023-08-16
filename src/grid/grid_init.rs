use rand::{rngs::StdRng, Rng, SeedableRng};
use super::cell::Cell;

pub struct CellRng {
    pub source: StdRng
}

impl CellRng {
    pub fn new(seed: &str) -> Self {
        let seed_bytes = seed.as_bytes().try_into().unwrap();
        let source = StdRng::from_seed(seed_bytes);
        Self { source: source }
    }
}

pub fn initialize_cell(cell: &mut Cell, x: usize, y: usize, rng: &mut CellRng){
    cell.set_xy(x, y);
    let is_alive: bool = rng.source.gen();
    cell.set_is_alive(is_alive);
}

pub fn can_initialize_cell(width: usize, height: usize, x: usize, y: usize, initial_field_size: usize) -> bool {
    let midpoint_x = height / 2;
    let midpoint_y = width / 2;

    let field_half_size = initial_field_size / 2;

    let size_left = midpoint_x - field_half_size;
    let size_right = midpoint_x + field_half_size;
    let size_up = midpoint_y - field_half_size;
    let size_down = midpoint_y + field_half_size;

    y > size_left && y < size_right && x > size_up && x < size_down
}