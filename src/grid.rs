mod cell;
mod directions;
use directions::Directions;
use cell::Cell;

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    directions: Vec<(i32, i32)>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = vec![vec![Cell::new(0,0); width]; height];

        for (index, sub_vector) in cells.iter_mut().enumerate() {
            for (sub_index, cell) in sub_vector.iter_mut().enumerate() {
                cell.set_xy(index as u32, sub_index as u32);
            }
        }
        Self { cells: cells, directions: Directions::get_all_directions() }
    }

    pub fn display_all(&self){
        for vec in self.cells.iter() {
            for cell in vec {
                cell.display();
            }
            println!("");
        }
    }

    #[allow(unused)]
    pub fn calculate_cell_life(&mut self){
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len(){
                let num_of_adjecent = self.calc_num_of_alive_adjecent(i as i32, j as i32);
                self.cells[i][j].set_alive_in_next_gen(num_of_adjecent);
            }
        }
    }

    #[allow(unused)]
    fn calc_num_of_alive_adjecent(&self, x: i32, y: i32) -> u32 {        
        let mut sum = 0;        

        for i in self.directions.iter() {
            //This will probably panic when -1 is added to the 0 index and then converted to usize
            let offset_x = (x + i.0) as usize;
            let offset_y = (y + i.1) as usize;

            if offset_x >= 0 && offset_y >= 0 {
                let cell = &self.cells[offset_x][offset_y];

                if cell.is_alive() {
                    sum += 1;
                }
            }
        }

        sum
    }
}