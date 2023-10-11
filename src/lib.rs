pub mod input;

use std::{hash::Hash, marker::PhantomData};

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
pub struct InputMapper<T>
where
    T: Copy + States,
{
    pub(crate) action_value: HashMap<(T, String), f32>,
    pub(crate) previous_action_value: HashMap<(T, String), f32>,

    pub(crate) keyboard_binding: HashMap<(T, KeyCode), String>,
    pub(crate) mouse_button_binding: HashMap<(T, MouseButton), String>,
    pub(crate) mouse_axis_binding: HashMap<(T, MouseAxis), String>,

    pub(crate) gamepad_axis_binding: HashMap<(T, GamepadAxis), String>,
    pub(crate) gamepad_button_binding: HashMap<(T, GamepadButtonType), String>,
}

impl<T> InputMapper<T>
where
    T: Copy + States,
{
    pub fn bind_keyboard_key_press(
        &mut self,
        scenario: T,
        key: KeyCode,
        action: impl ToString,
    ) -> &mut Self {
        self.keyboard_binding
            .bind((scenario, key), action.to_string());
        self
    }
    pub fn bind_mouse_axis_move(
        &mut self,
        scenario: T,
        axis: MouseAxis,
        action: impl ToString,
    ) -> &mut Self {
        self.mouse_axis_binding
            .bind((scenario, axis), action.to_string());
        self
    }
    pub fn bind_mouse_button_press(
        &mut self,
        scenario: T,
        button: MouseButton,
        action: impl ToString,
    ) -> &mut Self {
        self.mouse_button_binding
            .bind((scenario, button), action.to_string());
        self
    }
    pub fn bind_gamepad_axis_move(
        &mut self,
        scenario: T,
        axis: GamepadAxis,
        action: impl ToString,
    ) -> &mut Self {
        self.gamepad_axis_binding
            .bind((scenario, axis), action.to_string());
        self
    }
    pub fn bind_gamepad_button_press(
        &mut self,
        scenario: T,
        button: GamepadButtonType,
        action: impl ToString,
    ) -> &mut Self {
        self.gamepad_button_binding
            .bind((scenario, button), action.to_string());
        self
    }
}

#[derive(Default)]
pub struct InputMapperPlugin<T>
where
    T: Copy + States,
{
    _phantom: PhantomData<T>,
}

impl<T> InputMapperPlugin<T>
where
    T: Copy + States,
{
    pub fn new() -> Self {
        InputMapperPlugin {
            _phantom: PhantomData,
        }
    }
}

impl<T> Plugin for InputMapperPlugin<T>
where
    T: Copy + States,
{
    fn build(&self, app: &mut App) {
        app.insert_resource::<InputMapper<T>>(InputMapper::default())
            .add_event::<InputActionActive>()
            .add_event::<InputActionStarted>()
            .add_event::<InputActionContinuing>()
            .add_event::<InputActionFinished>()
            .add_systems(Update, InputMapper::<T>::event_cycle)
            .add_systems(Update, InputMapper::<T>::keyboard_key_press_system)
            .add_systems(
                Update,
                (
                    InputMapper::<T>::mouse_button_press_system,
                    InputMapper::<T>::mouse_axis_move_system,
                ),
            )
            .add_systems(
                Update,
                (
                    InputMapper::<T>::gamepad_button_press_system,
                    InputMapper::<T>::gamepad_axis_move_system,
                ),
            );
    }
}
