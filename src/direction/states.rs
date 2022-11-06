#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn safe_next(current: Direction, next: Direction) -> Direction {
        match (current, next) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Right, Direction::Left)
            | (Direction::Left, Direction::Right) => current,
            _ => next,
        }
    }
}
