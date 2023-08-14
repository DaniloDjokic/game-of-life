pub struct Directions {}

impl Directions {
    pub fn get_all_directions() -> Vec<(isize, isize)> {
        vec![(-1, 0), (1, 0), (0, 1), (0, -1),(1, -1), (1, 1), (-1, -1), (-1, 1)]
    }
}