use crate::terminal;

#[derive(Clone)]
pub struct Cell {
    x: u32,
    y: u32,
    is_alive: bool,
    is_alive_in_next_gen: bool
}

impl Cell {
    pub fn display(&self, x: usize, y: usize){
        terminal::display_cell(self.is_alive, x as u16, y as u16);
    }

    pub fn update_to_current_gen(&mut self) {
        self.is_alive = self.is_alive_in_next_gen;
    }

    pub fn new(x: u32, y: u32) -> Self {
        Self { x: x, y: y, is_alive: false, is_alive_in_next_gen: false }
    }

    pub fn set_xy(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn set_is_alive(&mut self, val: bool) {
        self.is_alive = val;
    }

    pub fn set_alive_in_next_gen(&mut self, adjecent_alive: u32){
        self.is_alive_in_next_gen = if self.is_alive {
            match adjecent_alive {
                0 | 1 => false,
                2 | 3 => true,
                _ => false,
            }
        }
        else {
            match adjecent_alive {
                3 => true,
                _ => false,
            }
        };
    }
}
