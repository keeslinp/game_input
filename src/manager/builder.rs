use manager::manager::*;
use std::hash::Hash;
pub struct ManagerBuilder<A: Hash + Eq, B: Hash + Eq, C: Hash + Eq> {
    manager: Manager<A, B, C>,
}

pub trait IManagerBuilder<A: Hash + Eq, B: Hash + Eq, C: Hash + Eq> {
    type Product;
    fn new() -> Self;
    fn build(self) -> Self::Product;
    fn add_axis_binding(self, axis: A, input: C) -> Self;
    fn add_button_binding(self, button: B, input: C) -> Self;
    fn add_default_change(self, impl Into<Change>, C) -> Self;
}

impl<A: Hash + Eq + Clone, B: Hash + Eq + Clone, C: Hash + Eq> IManagerBuilder<A, B, C>
    for ManagerBuilder<A, B, C>
{
    type Product = Manager<A, B, C>;
    fn new() -> ManagerBuilder<A, B, C> {
        ManagerBuilder {
            manager: Manager::new(),
        }
    }
    fn build(self) -> Manager<A, B, C> {
        self.manager
    }
    fn add_axis_binding(mut self, axis: A, input: C) -> Self {
        self.manager.add_axis_binding(axis, input);
        self
    }
    fn add_button_binding(mut self, button: B, input: C) -> Self {
        self.manager.add_button_binding(button, input);
        self
    }

    fn add_default_change(mut self, change: impl Into<Change>, input: C) -> Self {
        self.manager.add_default_change(change, input);
        self
    }
}
