pub struct ButtonChange(pub bool);
pub enum AxisChange {
    Position(f64),
    Velocity(f64),
}
