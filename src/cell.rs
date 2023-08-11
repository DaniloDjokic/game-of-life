#[derive(Clone)]
pub struct Cell {
    x: u32,
    y: u32,
    is_alive: bool
}

impl Cell {
    pub fn display(&self){
        if self.is_alive {
            print!("#");
        }
        else {
            print!(" ");
        }
    }

    pub fn new(x: u32, y: u32) -> Self {
        Self { x: x, y: y, is_alive: true }
    }

    pub fn set_xy(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}
