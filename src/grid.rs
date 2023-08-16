mod cell;
mod directions;
mod grid_init;
mod grid_display;
use directions::Directions;
use cell::Cell;
use grid_init::CellRng;

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    directions: Vec<(isize, isize)>,
}

impl Grid {
    pub fn new(width: usize, height: usize, seed: &str, initial_field_size: usize) -> Self {
        if initial_field_size >= height || initial_field_size >= width{
            panic!("Field size cannot be equal or higher than grid size");
        }

        let mut rng = CellRng::new(&seed);
        let cells = Grid::init_cell_grid(width, height, initial_field_size, &mut rng);
        
        Self { cells: cells, directions: Directions::get_all_directions() }
    }

    fn init_cell_grid(width: usize, height: usize, initial_field_size: usize, rng: &mut CellRng) -> Vec<Vec<Cell>> {
        let mut cells = vec![vec![Cell::new(0,0); height]; width];

        for (index, sub_vector) in cells.iter_mut().enumerate() {
            for (sub_index, cell) in sub_vector.iter_mut().enumerate() {
                if grid_init::can_initialize_cell(width, height, index, sub_index, initial_field_size){
                    grid_init::initialize_cell(cell, index, sub_index, rng);
                }
            }
        }

        cells
    }

    pub fn calculate_cell_life(&mut self){
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len(){
                let num_of_adjecent = self.calc_num_of_alive_adjecent(i, j);
                self.cells[i][j].set_alive_in_next_gen(num_of_adjecent);
            }
        }
    }

    fn calc_num_of_alive_adjecent(&self, x: usize, y: usize) -> u32 {        
        let mut sum = 0;        

        for i in self.directions.iter() {
            if self.is_out_of_bounds(x, y) {
                continue;
            }

            let offset_x = (x as isize) + i.0;
            let offset_y = (y as isize) + i.1;

            let cell = &self.cells[offset_x as usize][offset_y as usize];

            if cell.is_alive() {
                sum += 1;
            }
        }

        sum
    }

    fn is_out_of_bounds(&self, x: usize, y: usize) -> bool {
        x <= 0 || x >= self.cells.len() - 1 || y <= 0 || y >= self.cells[x].len() - 1 
    }
}