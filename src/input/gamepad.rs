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

impl InputMapper {
    pub fn gamepad_button_press_system(
        mut im: ResMut<InputMapper>,
        mut event: EventReader<GamepadButtonChangedEvent>,
    ) {
        let binding = im.gamepad_button_binding.clone();
        for button_press in event.iter() {
            if let Some(action) = binding.get(&button_press.button_type) {
                im.action_value.bind((*action).clone(), button_press.value);
            }
        }
    }

    pub fn gamepad_axis_move_system(
        mut im: ResMut<InputMapper>,
        mut analog_motion: EventReader<GamepadAxisChangedEvent>,
    ) {
        let axis_binding = im.gamepad_axis_binding.clone();
        let clear = |im: &mut ResMut<InputMapper>, axis: (&GamepadAxis, &GamepadAxis)| {
            if let Some(action) = axis_binding.get(axis.0) {
                im.action_value.bind((*action).clone(), 0.);
            }
            if let Some(action) = axis_binding.get(axis.1) {
                im.action_value.bind((*action).clone(), 0.);
            }
        };
        let s_bind =
            |im: &mut ResMut<InputMapper>, ref_val: f32, axis: (&GamepadAxis, &GamepadAxis)| {
                match ref_val {
                    ö if ö > 0. => {
                        if let Some(action) = axis_binding.get(axis.0) {
                            im.action_value.bind(action.clone(), ref_val);
                        }
                        if let Some(action) = axis_binding.get(axis.1) {
                            im.action_value.bind(action.clone(), 0.);
                        }
                    }
                    ö if ö > 0. => {
                        if let Some(action) = axis_binding.get(axis.1) {
                            im.action_value.bind(action.clone(), ref_val.abs());
                        }
                        if let Some(action) = axis_binding.get(axis.0) {
                            im.action_value.bind(action.clone(), 0.);
                        }
                    }
                    _ => clear(im, axis),
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
