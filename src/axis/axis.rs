use change::AxisChange;
use std::time::Duration;

#[derive(Default, Debug, PartialEq)]
pub struct Axis {
    pub position: f64,
    pub velocity: f64,
}

pub trait IAxis {
    fn apply(&mut self, AxisChange);
    fn tick(&mut self, Duration);
}

impl IAxis for Axis {
    fn apply(&mut self, change: AxisChange) {
        use AxisChange::*;
        match change {
            Position(pos) => self.position = pos,
            Velocity(vel) => self.velocity = vel,
        }
    }
    fn tick(&mut self, delta: Duration) {
        self.position += self.velocity
            * (delta.as_secs() * 1000 + (delta.subsec_millis() as u64)) as f64
            / 1000.0;
        self.position = if self.position > 1.0 {
            1.0
        } else if self.position < -1.0 {
            -1.0
        } else {
            self.position
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
        axis.apply(AxisChange::Velocity(0.5));
        assert_eq!(axis.velocity, 0.5);
    }
    mod tick {
        use super::*;
        #[test]
        fn can_tick() {
            let delta = Duration::from_millis(100);
            let mut axis = Axis {
                position: 0.0,
                velocity: 1.0,
            };
            axis.tick(delta);
            assert_eq!(axis.position, 0.1);

            axis.velocity = -1.0;
            axis.tick(delta * 2);
            assert_eq!(axis.position, -0.1);
        }

        #[test]
        fn caps_at_1() {
            let delta = Duration::from_millis(1000);
            let mut axis = Axis {
                position: 0.9,
                velocity: 1.0,
            };
            axis.tick(delta);
            assert_eq!(axis.position, 1.0);
        }
        #[test]
        fn caps_at_neg_1() {
            let delta = Duration::from_millis(1000);
            let mut axis = Axis {
                position: -0.9,
                velocity: -1.0,
            };
            axis.tick(delta);
            assert_eq!(axis.position, -1.0);
        }
    }
}
