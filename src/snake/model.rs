pub struct SnakeModel {
    pub pos: [f64; 2],
    pub speed: f64,
}

impl SnakeModel {
    pub fn new(starting_pos: [f64; 2], speed: f64) -> SnakeModel {
        SnakeModel {
            pos: starting_pos,
            speed,
        }
    }
}
