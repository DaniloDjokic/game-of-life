use super::cell::Cell;

pub struct Grid {
    cells: Vec<Vec<Cell>>
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = vec![vec![Cell::new(0,0); width]; height];

        for (index, sub_vector) in cells.iter_mut().enumerate() {
            for (sub_index, cell) in sub_vector.iter_mut().enumerate() {
                cell.set_xy(index as u32, sub_index as u32);
            }
        }

        Self { cells }
    }

    pub fn display_all(&self){
        for vec in self.cells.iter() {
            for cell in vec {
                cell.display();
            }
            println!("");
        }
    }
}