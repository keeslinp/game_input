mod axis;
mod button;
mod change;
mod manager;

pub use axis::*;
pub use button::*;
pub use change::*;
pub use manager::Manager;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
