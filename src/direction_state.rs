#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct DirectionState(Direction);

impl DirectionState {
    pub fn new() -> Self {
        DirectionState(Direction::Right)
    }

    pub fn get_value(&self) -> Direction {
        self.0
    }

    pub fn next_to(&mut self, value: Direction) {
        let DirectionState(current_direction) = self;

        match (current_direction, value) {
            (Direction::Up, Direction::Left) => {
                self.0 = Direction::Left;
            }
            (Direction::Up, Direction::Right) => {
                self.0 = Direction::Right;
            }
            (Direction::Down, Direction::Left) => {
                self.0 = Direction::Left;
            }
            (Direction::Down, Direction::Right) => {
                self.0 = Direction::Right;
            }
            (Direction::Left, Direction::Up) => {
                self.0 = Direction::Up;
            }
            (Direction::Left, Direction::Down) => {
                self.0 = Direction::Down;
            }
            (Direction::Right, Direction::Up) => {
                self.0 = Direction::Up;
            }
            (Direction::Right, Direction::Down) => {
                self.0 = Direction::Down;
            }
            (_, _) => {}
        }
    }
}
