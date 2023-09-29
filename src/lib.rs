pub mod input;

use std::hash::Hash;

use bevy::{prelude::*, utils::HashMap};
use input::{
    events::{InputActionContinuing, InputActionFinished, InputActionStarted},
    gamepad::GamepadAxis,
    mouse::MouseAxis,
};

use crate::input::events::InputActionActive;

pub(crate) trait AutoBinder<K, V>
where
    K: Eq + Hash,
{
    fn bind(&mut self, key: K, value: V) -> &mut Self;
    fn unbind(&mut self, key: K) -> &mut Self;
}

impl<K, V> AutoBinder<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn bind(&mut self, key: K, value: V) -> &mut Self {
        self.insert(key, value);
        self
    }

    fn unbind(&mut self, key: K) -> &mut Self {
        self.remove(&key);
        self
    }
}

#[derive(Default, Clone, Resource)]
pub struct InputMapper {
    pub(crate) action_value: HashMap<String, f32>,
    pub(crate) previous_action_value: HashMap<String, f32>,

    pub(crate) keyboard_binding: HashMap<KeyCode, String>,
    pub(crate) mouse_button_binding: HashMap<MouseButton, String>,
    pub(crate) mouse_axis_binding: HashMap<MouseAxis, String>,

    pub(crate) gamepad_axis_binding: HashMap<GamepadAxis, String>,
    pub(crate) gamepad_button_binding: HashMap<GamepadButtonType, String>,
}

impl InputMapper {
    pub fn bind_keyboard_key_press(&mut self, key: KeyCode, action: impl ToString) -> &mut Self {
        self.keyboard_binding.bind(key, action.to_string());
        self
    }
    pub fn bind_mouse_axis_move(&mut self, axis: MouseAxis, action: impl ToString) -> &mut Self {
        self.mouse_axis_binding.bind(axis, action.to_string());
        self
    }
    pub fn bind_mouse_button_press(
        &mut self,
        button: MouseButton,
        action: impl ToString,
    ) -> &mut Self {
        self.mouse_button_binding.bind(button, action.to_string());
        self
    }
    pub fn bind_gamepad_axis_move(
        &mut self,
        axis: GamepadAxis,
        action: impl ToString,
    ) -> &mut Self {
        self.gamepad_axis_binding.bind(axis, action.to_string());
        self
    }
    pub fn bind_gamepad_button_press(
        &mut self,
        button: GamepadButtonType,
        action: impl ToString,
    ) -> &mut Self {
        self.gamepad_button_binding.bind(button, action.to_string());
        self
    }
}

#[derive(Default)]
pub struct InputMapperPlugin;

impl Plugin for InputMapperPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputMapper::default())
            .add_event::<InputActionActive>()
            .add_event::<InputActionStarted>()
            .add_event::<InputActionContinuing>()
            .add_event::<InputActionFinished>()
            .add_systems(Update, InputMapper::event_cycle)
            .add_systems(Update, InputMapper::keyboard_key_press_system)
            .add_systems(
                Update,
                (
                    InputMapper::mouse_button_press_system,
                    InputMapper::mouse_axis_move_system,
                ),
            )
            .add_systems(
                Update,
                (
                    InputMapper::gamepad_button_press_system,
                    InputMapper::gamepad_axis_move_system,
                ),
            );
    }
}
