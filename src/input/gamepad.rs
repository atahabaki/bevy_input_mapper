use bevy::{
    input::gamepad::{GamepadAxisChangedEvent, GamepadButtonChangedEvent},
    prelude::*,
};

use crate::{AutoBinder, InputMapper};

/// Represents gamepad' analog sticks' movement.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum GamepadAxis {
    /// Left Analog X axis, positive movement.
    PositiveLeftStickX,
    /// Left Analog X axis, negative movement.
    NegativeLeftStickX,
    /// Left Analog Y axis, positive movement.
    PositiveLeftStickY,
    /// Left Analog Y axis, negative movement.
    NegativeLeftStickY,
    /// Right Analog X axis, positive movement.
    PositiveRightStickX,
    /// Right Analog X axis, negative movement.
    NegativeRightStickX,
    /// Right Analog Y axis, positive movement.
    PositiveRightStickY,
    /// Right Analog Y axis, negative movement.
    NegativeRightStickY,
    /// Left Z button's positive values.
    PositiveLeftZ,
    /// Left Z button's negative values.
    NegativeLeftZ,
    /// Right Z button's negative values.
    PositiveRightZ,
    /// Right Z button's negative values.
    NegativeRightZ,
    /// Other
    PositiveOtherAxis(u8),
    NegativeOtherAxis(u8),
}

impl<T> InputMapper<T>
where
    T: Copy + States,
{
    pub(crate) fn gamepad_button_press_system(
        mut im: ResMut<InputMapper<T>>,
        state: Res<State<T>>,
        mut event: EventReader<GamepadButtonChangedEvent>,
    ) {
        let binding = im.gamepad_button_binding.clone();
        let current_state = *state.get();
        for button_press in event.iter() {
            if let Some(action) = binding.get(&(current_state, button_press.button_type)) {
                im.action_value
                    .bind((current_state, (*action).clone()), button_press.value);
            }
        }
    }

    pub(crate) fn gamepad_axis_move_system(
        mut im: ResMut<InputMapper<T>>,
        state: Res<State<T>>,
        mut analog_motion: EventReader<GamepadAxisChangedEvent>,
    ) {
        let axis_binding = im.gamepad_axis_binding.clone();
        let current_state = *state.get();
        let set_val = |im: &mut ResMut<InputMapper<T>>,
                       axis: (&GamepadAxis, &GamepadAxis),
                       val: (f32, f32)| {
            if let Some(action) = axis_binding.get(&(current_state, axis.0.clone())) {
                im.action_value
                    .bind((current_state, (*action).clone()), val.0);
            }
            if let Some(action) = axis_binding.get(&(current_state, axis.1.clone())) {
                im.action_value
                    .bind((current_state, (*action).clone()), val.1);
            }
        };
        let s_bind =
            |im: &mut ResMut<InputMapper<T>>, ref_val: f32, axis: (&GamepadAxis, &GamepadAxis)| {
                match ref_val {
                    ö if ö > 0. => set_val(im, axis, (ref_val, 0.)),
                    // idk if it makes any difference
                    // Option 1:
                    // ö if ö < 0. => set_val(im, axis, (0., ref_val.abs())),
                    // Option 2:
                    // Which one is better, idk... I don't think it will make any difference anyway.
                    // If it does, pls. create an issue or make a PR.
                    ö if ö < 0. => set_val(im, (axis.1, axis.0), (ref_val.abs(), 0.)),
                    _ => set_val(im, axis, (0., 0.)),
                }
            };
        for motion in analog_motion.iter() {
            match motion.axis_type {
                GamepadAxisType::LeftStickX => s_bind(
                    &mut im,
                    motion.value,
                    (
                        &GamepadAxis::PositiveLeftStickX,
                        &GamepadAxis::NegativeLeftStickX,
                    ),
                ),
                GamepadAxisType::LeftStickY => s_bind(
                    &mut im,
                    motion.value,
                    (
                        &GamepadAxis::PositiveLeftStickY,
                        &GamepadAxis::NegativeLeftStickY,
                    ),
                ),
                GamepadAxisType::LeftZ => s_bind(
                    &mut im,
                    motion.value,
                    (&GamepadAxis::PositiveLeftZ, &GamepadAxis::NegativeLeftZ),
                ),
                GamepadAxisType::RightStickX => s_bind(
                    &mut im,
                    motion.value,
                    (
                        &GamepadAxis::PositiveRightStickX,
                        &GamepadAxis::NegativeRightStickX,
                    ),
                ),
                GamepadAxisType::RightStickY => s_bind(
                    &mut im,
                    motion.value,
                    (
                        &GamepadAxis::PositiveRightStickY,
                        &GamepadAxis::NegativeRightStickY,
                    ),
                ),
                GamepadAxisType::RightZ => s_bind(
                    &mut im,
                    motion.value,
                    (&GamepadAxis::PositiveRightZ, &GamepadAxis::NegativeRightZ),
                ),
                GamepadAxisType::Other(v) => s_bind(
                    &mut im,
                    motion.value,
                    (
                        &GamepadAxis::PositiveOtherAxis(v),
                        &GamepadAxis::NegativeOtherAxis(v),
                    ),
                ),
            }
        }
    }
}
