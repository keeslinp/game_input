use axis::*;
use button::*;
use change::{AxisChange, ButtonChange, Direction};
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;

#[derive(Clone)]
pub enum Change {
    Axis(AxisChange),
    Button(ButtonChange),
}

impl Into<Change> for AxisChange {
    fn into(self) -> Change {
        Change::Axis(self)
    }
}

impl Into<Change> for ButtonChange {
    fn into(self) -> Change {
        Change::Button(self)
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Binding<A, B> {
    Axis(A),
    Button(B),
}

pub enum State {
    Axis(Axis),
    Button(Button),
}

pub struct Manager<A: Hash + Eq, B: Hash + Eq, C: Hash + Eq> {
    states: HashMap<Binding<A, B>, State>,
    bindings: HashMap<C, Binding<A, B>>,
    default_changes: HashMap<C, Change>,
}

pub trait IManager<A, B, C> {
    fn new() -> Self;
    fn get_axis(&self, binding: A) -> Option<&Axis>;
    fn get_button(&self, binding: B) -> Option<&Button>;
    fn get_states(&self) -> &HashMap<Binding<A, B>, State>;
    fn get_changed_buttons(&self) -> HashMap<&B, &Button>;
    fn get_button_pressed(&self, B) -> bool;
}

pub trait IConverter<A, B, C> {
    fn add_axis_binding(&mut self, axis: A, input: C);
    fn get_binding(&self, C) -> Option<Binding<A, B>>;
    fn add_button_binding(&mut self, button: B, input: C);
    fn get_default_change(&self, C) -> Option<Change>;
    fn add_default_change(&mut self, impl Into<Change>, C);
}

pub trait IUpdater<A, B> {
    fn tick(&mut self, Duration);
    fn apply_change(&mut self, &Binding<A, B>, impl Into<Change>);
}

impl<A: Hash + Eq + Clone, B: Hash + Eq + Clone, C: Hash + Eq> IUpdater<A, B> for Manager<A, B, C> {
    fn tick(&mut self, delta: Duration) {
        for state in self.states.values_mut() {
            match state {
                State::Axis(axis) => axis.tick(delta),
                State::Button(button) => button.tick(),
            }
        }
    }
    fn apply_change(&mut self, binding: &Binding<A, B>, change: impl Into<Change>) {
        if let Some(ref mut state) = self.states.get_mut(binding) {
            match (state, change.into()) {
                (&mut State::Axis(ref mut a), Change::Axis(ref c)) => a.apply(c.clone()),
                (&mut State::Button(ref mut b), Change::Button(ref c)) => b.apply(c.clone()),
                _ => unreachable!(),
            }
        }
    }
}

impl<A: Hash + Eq + Clone, B: Hash + Eq + Clone, C: Hash + Eq> IConverter<A, B, C>
    for Manager<A, B, C>
{
    fn add_axis_binding(&mut self, axis: A, input: C) {
        let binding = Binding::Axis(axis);
        self.bindings.insert(input, binding.clone());
        if !self.states.contains_key(&binding) {
            self.states.insert(binding, State::Axis(Axis::default()));
        }
    }

    fn add_default_change(&mut self, change: impl Into<Change>, input: C) {
        self.default_changes.insert(input, change.into());
    }

    fn get_binding(&self, input: C) -> Option<Binding<A, B>> {
        self.bindings.get(&input).cloned()
    }
    fn get_default_change(&self, input: C) -> Option<Change> {
        self.default_changes.get(&input).cloned()
    }

    fn add_button_binding(&mut self, button: B, input: C) {
        let binding = Binding::Button(button);
        self.bindings.insert(input, binding.clone());
        if !self.states.contains_key(&binding) {
            self.states
                .insert(binding, State::Button(Button::default()));
        }
    }
}

impl<A: Hash + Eq + Clone, B: Hash + Eq + Clone, C: Hash + Eq> IManager<A, B, C>
    for Manager<A, B, C>
{
    fn new() -> Self {
        Manager {
            states: HashMap::new(),
            bindings: HashMap::new(),
            default_changes: HashMap::new(),
        }
    }

    fn get_axis(&self, binding: A) -> Option<&Axis> {
        self.states.get(&Binding::Axis(binding)).and_then(|val| {
            if let State::Axis(a) = val {
                Some(a)
            } else {
                None
            }
        })
    }
    fn get_states(&self) -> &HashMap<Binding<A, B>, State> {
        &self.states
    }

    fn get_button(&self, binding: B) -> Option<&Button> {
        self.states.get(&Binding::Button(binding)).and_then(|val| {
            if let State::Button(b) = val {
                Some(b)
            } else {
                None
            }
        })
    }
    fn get_changed_buttons(&self) -> HashMap<&B, &Button> {
        self.states
            .iter()
            .filter_map(|(key, val)| {
                if let State::Button(b) = val {
                    if b.new_event {
                        if let Binding::Button(bind) = key {
                            Some((bind, b))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_button_pressed(&self, button: B) -> bool {
        self.get_button(button)
            .map(|button| button.pressed && button.new_event)
            .unwrap_or(false)
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
        assert_eq!(manager.bindings.len(), 1);
        assert_eq!(
            manager.get_binding(Input::Button(Keyboard::A)).unwrap(),
            Binding::Axis(Axes::Vertical)
        );
        assert_eq!(manager.get_axis(Axes::Vertical).unwrap(), &Axis::default());
    }
    #[test]
    fn can_add_button() {
        let mut manager: Manager<Axes, Buttons, Input> = Manager::new();
        manager.add_button_binding(Buttons::Fire, Input::Button(Keyboard::A));
        assert_eq!(manager.states.len(), 1);
        assert_eq!(
            manager.get_binding(Input::Button(Keyboard::A)).unwrap(),
            Binding::Button(Buttons::Fire)
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
        let binding = manager.get_binding(Input::Button(Keyboard::A)).unwrap();
        manager.apply_change(&binding, AxisChange::Position(0.5));
        assert_eq!(manager.get_axis(Axes::Vertical).unwrap().position, 0.5);
    }
    #[test]
    fn can_update_axis_vel() {
        let mut manager: Manager<Axes, Buttons, Input> = Manager::new();
        manager.add_axis_binding(Axes::Vertical, Input::Button(Keyboard::A));
        let binding = &manager
            .get_binding(Input::Button(Keyboard::A))
            .unwrap()
            .clone();
        manager.apply_change(binding, AxisChange::Velocity(Direction::Up));
        assert_eq!(
            manager.get_axis(Axes::Vertical).unwrap().velocity,
            Some(Direction::Up)
        );
    }

    #[test]
    fn can_toggle_button() {
        let mut manager: Manager<Axes, Buttons, Input> = Manager::new();
        manager.add_button_binding(Buttons::Fire, Input::Button(Keyboard::A));
        let binding = &manager
            .get_binding(Input::Button(Keyboard::A))
            .unwrap()
            .clone();
        manager.apply_change(binding, ButtonChange(true));
        assert_eq!(manager.get_button(Buttons::Fire).unwrap().pressed, true);
        let binding = &manager
            .get_binding(Input::Button(Keyboard::A))
            .unwrap()
            .clone();
        manager.apply_change(binding, ButtonChange(false));
        assert_eq!(manager.get_button(Buttons::Fire).unwrap().pressed, false);
    }

    #[test]
    fn get_changed_buttons() {
        let mut manager: Manager<Axes, Buttons, Input> = Manager::new();
        manager.add_button_binding(Buttons::Fire, Input::Button(Keyboard::A));
        manager.add_button_binding(Buttons::Block, Input::Button(Keyboard::B));
        let binding = &manager
            .get_binding(Input::Button(Keyboard::A))
            .unwrap()
            .clone();
        manager.apply_change(binding, ButtonChange(true));
        assert_eq!(manager.get_changed_buttons().len(), 1);
        manager.tick(Duration::default());
        assert_eq!(manager.get_changed_buttons().len(), 0);
    }
}
