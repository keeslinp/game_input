use change::ButtonChange;

#[derive(Default, Debug, PartialEq)]
pub struct Button {
    pub pressed: bool,
    pub new_event: bool,
}

pub trait IButton {
    fn apply(&mut self, ButtonChange);
    fn tick(&mut self);
}

impl IButton for Button {
    fn apply(&mut self, change: ButtonChange) {
        if self.pressed != change.0 {
            self.new_event = true;
        }
        self.pressed = change.0;
    }
    fn tick(&mut self) {
        self.new_event = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_apply() {
        let mut button = Button::default();
        button.apply(ButtonChange(true));
        assert_eq!(button.pressed, true);
        button.apply(ButtonChange(false));
        assert_eq!(button.pressed, false);
    }
    #[test]
    fn new_event_on_change() {
        let mut button = Button::default();
        button.apply(ButtonChange(true));
        assert_eq!(button.new_event, true);
        button.apply(ButtonChange(false));
        assert_eq!(button.new_event, true);
    }
    #[test]
    fn new_event_only_on_change() {
        let mut button = Button::default();
        button.apply(ButtonChange(false));
        assert_eq!(button.new_event, false);
        button = Button {
            pressed: true,
            new_event: false,
        };
        button.apply(ButtonChange(true));
        assert_eq!(button.new_event, false);
    }
    #[test]
    fn new_event_cleared_by_tick() {
        let mut button = Button {
            pressed: true,
            new_event: true,
        };
        button.tick();
        assert_eq!(button.new_event, false);
    }
}
