impl super::Grid {
    pub fn display_all(&self) {
        for (idx, row) in self.cells.iter().enumerate() {
            for (sub_idx, cell) in row.iter().enumerate() {
                cell.display(idx, sub_idx);
            }  
        }
    }

    pub fn display_all_and_update(&mut self){
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len(){
                self.cells[i][j].display(i, j);
                self.cells[i][j].update_to_current_gen();
            }
        }
    }
}