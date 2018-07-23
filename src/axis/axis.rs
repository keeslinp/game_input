use change::{AxisChange, Direction};
use std::time::Duration;

#[derive(Default, Debug, PartialEq)]
pub struct Axis {
    pub position: f64,
    pub velocity: Option<Direction>,
    falling: bool,
}

pub trait IAxis {
    fn apply(&mut self, AxisChange);
    fn tick(&mut self, Duration);
}

impl IAxis for Axis {
    fn apply(&mut self, change: AxisChange) {
        self.falling = false;
        use AxisChange::*;
        match change {
            Position(pos) => self.position = pos,
            Velocity(vel) => self.velocity = Some(vel),
            Falling(dir) => {
                use Direction::*;
                match (dir, &self.velocity) {
                    (Down, Some(Down)) | (Up, Some(Up)) => {
                        self.falling = true;
                        self.velocity = None;
                    }
                    _ => {}
                }
            }
        }
    }
    fn tick(&mut self, delta: Duration) {
        let dx = if self.falling {
            if self.position > 0.0 {
                -1.0
            } else {
                1.0
            }
        } else {
            self.velocity
                .as_ref()
                .cloned()
                .map(|val| val.into())
                .unwrap_or(0.0)
        };
        self.position +=
            dx * (delta.as_secs() * 1000 + (delta.subsec_millis() as u64)) as f64 / 500.0;
        self.position = if self.falling {
            if dx * self.position > 0.0 {
                self.falling = false;
                0.0
            } else {
                self.position
            }
        } else {
            if self.position > 1.0 {
                1.0
            } else if self.position < -1.0 {
                -1.0
            } else {
                self.position
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_apply_pos() {
        let mut axis = Axis::default();
        axis.apply(AxisChange::Position(0.5));
        assert_eq!(axis.position, 0.5);
    }
    #[test]
    fn can_apply_vel() {
        let mut axis = Axis::default();
        axis.apply(AxisChange::Velocity(Direction::Up));
        assert_eq!(axis.velocity, Some(Direction::Up));
    }
    #[test]
    fn can_apply_falling() {
        let mut axis = Axis::default();
        axis.apply(AxisChange::Velocity(Direction::Up));
        axis.apply(AxisChange::Falling(Direction::Up));
        assert_eq!(axis.falling, true);
    }
    #[test]
    fn reset_velocity_on_fall() {
        let mut axis = Axis::default();
        axis.velocity = Some(Direction::Up);
        axis.position = 0.5;
        axis.apply(AxisChange::Falling(Direction::Up));
        assert_eq!(axis.velocity, None);
    }

    #[test]
    fn ignores_fall_when_moving_other_direction() {
        let mut axis = Axis::default();
        axis.velocity = Some(Direction::Up);
        axis.apply(AxisChange::Falling(Direction::Down));
        assert_eq!(axis.velocity, Some(Direction::Up));
        assert_eq!(axis.falling, false);
    }
    mod tick {
        use super::*;
        #[test]
        fn can_tick() {
            let delta = Duration::from_millis(100);
            let mut axis = Axis {
                position: 0.0,
                velocity: Some(Direction::Up),
                falling: false,
            };
            axis.tick(delta);
            assert_eq!(axis.position, 0.2);

            axis.velocity = Some(Direction::Down);
            axis.tick(delta * 2);
            assert_eq!(axis.position, -0.2);
        }

        #[test]
        fn caps_at_1() {
            let delta = Duration::from_millis(1000);
            let mut axis = Axis {
                position: 0.9,
                velocity: Some(Direction::Up),
                falling: false,
            };
            axis.tick(delta);
            assert_eq!(axis.position, 1.0);
        }
        #[test]
        fn caps_at_neg_1() {
            let delta = Duration::from_millis(1000);
            let mut axis = Axis {
                position: -0.9,
                velocity: Some(Direction::Down),
                falling: false,
            };
            axis.tick(delta);
            assert_eq!(axis.position, -1.0);
        }
        #[test]
        fn falling() {
            let delta = Duration::from_millis(100);
            let mut axis = Axis {
                position: 0.5,
                velocity: None,
                falling: true,
            };
            axis.tick(delta);
            assert_eq!(axis.position, 0.3);
        }

        #[test]
        fn can_settle() {
            let delta = Duration::from_millis(1000);
            let mut axis = Axis {
                position: 0.5,
                velocity: None,
                falling: true,
            };
            axis.tick(delta);
            assert_eq!(axis.position, 0.0);
        }
    }
}
