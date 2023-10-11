use bevy::prelude::*;

use crate::InputMapper;

impl<T> InputMapper<T>
where
    T: Copy + States,
{
    pub(crate) fn get_previous_value(&self, state: T, action: &str) -> &f32 {
        self.previous_action_value
            .get(&(state, action.to_owned()))
            .unwrap_or(&0.)
    }
    pub(crate) fn get_current_value(&self, state: T, action: &str) -> &f32 {
        self.action_value
            .get(&(state, action.to_owned()))
            .unwrap_or(&0.)
    }
    pub(crate) fn is_started(&self, state: T, action: &str) -> bool {
        self.get_previous_value(state, action) == &0. && self.get_current_value(state, action) > &0.
    }
    pub(crate) fn is_continuing(&self, state: T, action: &str) -> bool {
        self.get_previous_value(state, action) > &0. && self.get_current_value(state, action) > &0.
    }
    pub(crate) fn is_finished(&self, state: T, action: &str) -> bool {
        self.get_previous_value(state, action) > &0. && self.get_current_value(state, action) == &0.
    }
    pub(crate) fn is_active(&self, state: T, action: &str) -> bool {
        self.is_started(state, action)
            || self.is_continuing(state, action)
            || self.is_finished(state, action)
    }
}
