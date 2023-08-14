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

    pub fn is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn set_is_alive(&mut self, val: bool) {
        self.is_alive = val;
    }

    pub fn set_alive_in_next_gen(&mut self, adjecent_alive: u32){
        let alive = if self.is_alive {
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

        self.is_alive = alive;
    }
}
