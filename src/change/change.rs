#[derive(Clone)]
pub struct ButtonChange(pub bool);
#[derive(Clone)]
pub enum AxisChange {
    Position(f64),
    Velocity(Direction),
    Falling(Direction),
}

impl Into<f64> for Direction {
    fn into(self) -> f64 {
        match self {
            Direction::Up => 1.0,
            Direction::Down => -1.0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
}
