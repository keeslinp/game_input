use axis::*;
use button::*;
use change::{AxisChange, ButtonChange};
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;

pub struct Manager<A: Hash + Eq, B: Hash + Eq, C: Hash + Eq> {
    axis_states: HashMap<A, Axis>,
    button_states: HashMap<B, Button>,
    axis_bindings: HashMap<C, A>,
    button_bindings: HashMap<C, B>,
}

pub trait IManager<A, B, C> {
    fn new() -> Self;
    fn get_axis(&self, binding: A) -> Option<&Axis>;
    fn get_button(&self, binding: B) -> Option<&Button>;
    fn get_axes(&self) -> &HashMap<A, Axis>;
    fn get_buttons(&self) -> &HashMap<B, Button>;
    fn get_changed_buttons(&self) -> HashMap<&B, &Button>;
}

pub trait IConverter<A, B, C> {
    fn add_axis_binding(&mut self, axis: A, input: C);
    fn get_axis_binding(&self, input: C) -> Option<&A>;
    fn add_button_binding(&mut self, button: B, input: C);
    fn get_button_binding(&self, input: C) -> Option<&B>;
}

pub trait IUpdater<A, B> {
    fn tick(&mut self, Duration);
    fn apply_axis_change(&mut self, A, AxisChange);
    fn apply_button_change(&mut self, B, ButtonChange);
}

impl<A: Hash + Eq + Clone, B: Hash + Eq + Clone, C: Hash + Eq> IUpdater<A, B> for Manager<A, B, C> {
    fn tick(&mut self, delta: Duration) {
        for axis in self.axis_states.values_mut() {
            axis.tick(delta);
        }

        for button in self.button_states.values_mut() {
            button.tick();
        }
    }
    fn apply_axis_change(&mut self, axis: A, change: AxisChange) {
        if let Some(state) = self.axis_states.get_mut(&axis) {
            state.apply(change);
        }
    }
    fn apply_button_change(&mut self, button: B, change: ButtonChange) {
        if let Some(state) = self.button_states.get_mut(&button) {
            state.apply(change);
        }
    }
}

impl<A: Hash + Eq + Clone, B: Hash + Eq + Clone, C: Hash + Eq> IConverter<A, B, C>
    for Manager<A, B, C>
{
    fn add_axis_binding(&mut self, axis: A, input: C) {
        self.axis_bindings.insert(input, axis.clone());
        if !self.axis_states.contains_key(&axis) {
            self.axis_states.insert(axis, Axis::default());
        }
    }

    fn get_axis_binding(&self, input: C) -> Option<&A> {
        self.axis_bindings.get(&input)
    }

    fn add_button_binding(&mut self, button: B, input: C) {
        self.button_bindings.insert(input, button.clone());
        if !self.button_states.contains_key(&button) {
            self.button_states.insert(button, Button::default());
        }
    }

    fn get_button_binding(&self, input: C) -> Option<&B> {
        self.button_bindings.get(&input)
    }
}

impl<A: Hash + Eq + Clone, B: Hash + Eq + Clone, C: Hash + Eq> IManager<A, B, C>
    for Manager<A, B, C>
{
    fn new() -> Self {
        Manager {
            axis_states: HashMap::new(),
            button_states: HashMap::new(),
            axis_bindings: HashMap::new(),
            button_bindings: HashMap::new(),
        }
    }

    fn get_axis(&self, binding: A) -> Option<&Axis> {
        self.axis_states.get(&binding)
    }
    fn get_axes(&self) -> &HashMap<A, Axis> { &self.axis_states }

    fn get_button(&self, binding: B) -> Option<&Button> {
        self.button_states.get(&binding)
    }
    fn get_buttons(&self) -> &HashMap<B, Button> { &self.button_states }
    fn get_changed_buttons(&self) -> HashMap<&B, &Button> {
        self.button_states.iter().filter(|(_key, val)| val.new_event).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(PartialEq, Eq, Hash, Debug, Clone)]
    enum Axes {
        Vertical,
    }
    #[derive(PartialEq, Eq, Hash, Debug, Clone)]
    enum Buttons {
        Fire,
        Block,
    }
    #[derive(PartialEq, Eq, Hash, Debug, Clone)]
    enum Keyboard {
        A,
        B,
    }
    #[derive(PartialEq, Eq, Hash, Debug, Clone)]
    enum GamePadInput {
        Left,
        Right,
    }
    #[derive(PartialEq, Eq, Hash, Debug, Clone)]
    enum Input {
        Button(Keyboard),
        Gamepad(GamePadInput),
    }
    #[test]
    fn can_add_axis() {
        let mut manager: Manager<Axes, Buttons, Input> = Manager::new();
        manager.add_axis_binding(Axes::Vertical, Input::Button(Keyboard::A));
        assert_eq!(manager.axis_bindings.len(), 1);
        assert_eq!(
            manager
                .get_axis_binding(Input::Button(Keyboard::A))
                .unwrap(),
            &Axes::Vertical
        );
        assert_eq!(manager.get_axis(Axes::Vertical).unwrap(), &Axis::default());
    }
    #[test]
    fn can_add_button() {
        let mut manager: Manager<Axes, Buttons, Input> = Manager::new();
        manager.add_button_binding(Buttons::Fire, Input::Button(Keyboard::A));
        assert_eq!(manager.button_bindings.len(), 1);
        assert_eq!(
            manager
                .get_button_binding(Input::Button(Keyboard::A))
                .unwrap(),
            &Buttons::Fire
        );
        assert_eq!(
            manager.get_button(Buttons::Fire).unwrap(),
            &Button::default()
        );
    }

    #[test]
    fn can_update_axis_pos() {
        let mut manager: Manager<Axes, Buttons, Input> = Manager::new();
        manager.add_axis_binding(Axes::Vertical, Input::Button(Keyboard::A));
        manager.apply_axis_change(Axes::Vertical, AxisChange::Position(0.5));
        assert_eq!(manager.get_axis(Axes::Vertical).unwrap().position, 0.5);
    }
    #[test]
    fn can_update_axis_vel() {
        let mut manager: Manager<Axes, Buttons, Input> = Manager::new();
        manager.add_axis_binding(Axes::Vertical, Input::Button(Keyboard::A));
        manager.apply_axis_change(Axes::Vertical, AxisChange::Velocity(0.5));
        assert_eq!(manager.get_axis(Axes::Vertical).unwrap().velocity, 0.5);
    }

    #[test]
    fn can_toggle_button() {
        let mut manager: Manager<Axes, Buttons, Input> = Manager::new();
        manager.add_button_binding(Buttons::Fire, Input::Button(Keyboard::A));
        manager.apply_button_change(Buttons::Fire, ButtonChange(true));
        assert_eq!(manager.get_button(Buttons::Fire).unwrap().pressed, true);
        manager.apply_button_change(Buttons::Fire, ButtonChange(false));
        assert_eq!(manager.get_button(Buttons::Fire).unwrap().pressed, false);
    }

    #[test]
    fn get_changed_buttons() {
        let mut manager: Manager<Axes, Buttons, Input> = Manager::new();
        manager.add_button_binding(Buttons::Fire, Input::Button(Keyboard::A));
        manager.add_button_binding(Buttons::Block, Input::Button(Keyboard::B));
        manager.apply_button_change(Buttons::Fire, ButtonChange(true));
        assert_eq!(manager.get_changed_buttons().len(), 1);
        manager.tick(Duration::default());
        assert_eq!(manager.get_changed_buttons().len(), 0);
    }
}
