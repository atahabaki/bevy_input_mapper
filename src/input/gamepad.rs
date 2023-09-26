use bevy::{input::gamepad::{GamepadAxisChangedEvent, GamepadButtonChangedEvent}, prelude::*};

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
}

impl InputMapper {
    pub fn gamepad_button_press_system(
        mut im: ResMut<InputMapper>,
        mut event: EventReader<GamepadButtonChangedEvent>
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
        for motion in analog_motion.iter() {
            match motion.axis_type {
                GamepadAxisType::LeftStickX => match motion.value {
                    ö if ö > 0. => {
                        if let Some(action) = axis_binding.get(&GamepadAxis::PositiveLeftStickX) {
                            im.action_value.bind(action.clone(), motion.value);
                        }
                        if let Some(action) = axis_binding.get(&GamepadAxis::NegativeLeftStickX) {
                            im.action_value.bind(action.clone(), 0.);
                        }
                    }
                    ö if ö < 0. => {
                        if let Some(action) = axis_binding.get(&GamepadAxis::NegativeLeftStickX) {
                            im.action_value.bind(action.clone(), motion.value.abs());
                        }
                        if let Some(action) = axis_binding.get(&GamepadAxis::PositiveLeftStickX) {
                            im.action_value.bind(action.clone(), 0.);
                        }
                    }
                    _ => clear(
                        &mut im,
                        (
                            &GamepadAxis::PositiveLeftStickX,
                            &GamepadAxis::NegativeLeftStickX,
                        ),
                    ),
                },
                GamepadAxisType::LeftStickY => match motion.value {
                    ö if ö > 0. => {
                        if let Some(action) = axis_binding.get(&GamepadAxis::PositiveLeftStickY) {
                            im.action_value.bind(action.clone(), motion.value);
                        }
                        if let Some(action) = axis_binding.get(&GamepadAxis::NegativeLeftStickY) {
                            im.action_value.bind(action.clone(), 0.);
                        }
                    }
                    ö if ö < 0. => {
                        if let Some(action) = axis_binding.get(&GamepadAxis::NegativeLeftStickY) {
                            im.action_value.bind(action.clone(), motion.value.abs());
                        }
                        if let Some(action) = axis_binding.get(&GamepadAxis::PositiveLeftStickY) {
                            im.action_value.bind(action.clone(), 0.);
                        }
                    }
                    _ => clear(
                        &mut im,
                        (
                            &GamepadAxis::PositiveLeftStickY,
                            &GamepadAxis::NegativeLeftStickY,
                        ),
                    ),
                },
                GamepadAxisType::LeftZ => todo!(),
                GamepadAxisType::RightStickX => match motion.value {
                    ö if ö > 0. => {
                        if let Some(action) = axis_binding.get(&GamepadAxis::PositiveRightStickX) {
                            im.action_value.bind(action.clone(), motion.value);
                        }
                        if let Some(action) = axis_binding.get(&GamepadAxis::NegativeRightStickX) {
                            im.action_value.bind(action.clone(), 0.);
                        }
                    }
                    ö if ö < 0. => {
                        if let Some(action) = axis_binding.get(&GamepadAxis::NegativeRightStickX) {
                            im.action_value.bind(action.clone(), motion.value.abs());
                        }
                        if let Some(action) = axis_binding.get(&GamepadAxis::PositiveRightStickX) {
                            im.action_value.bind(action.clone(), 0.);
                        }
                    }
                    _ => clear(
                        &mut im,
                        (
                            &GamepadAxis::PositiveRightStickX,
                            &GamepadAxis::NegativeRightStickX,
                        ),
                    ),
                },
                GamepadAxisType::RightStickY => match motion.value {
                    ö if ö > 0. => {
                        if let Some(action) = axis_binding.get(&GamepadAxis::PositiveRightStickY) {
                            im.action_value.bind(action.clone(), motion.value);
                        }
                        if let Some(action) = axis_binding.get(&GamepadAxis::NegativeRightStickY) {
                            im.action_value.bind(action.clone(), 0.);
                        }
                    }
                    ö if ö < 0. => {
                        if let Some(action) = axis_binding.get(&GamepadAxis::NegativeRightStickY) {
                            im.action_value.bind(action.clone(), motion.value.abs());
                        }
                        if let Some(action) = axis_binding.get(&GamepadAxis::PositiveRightStickY) {
                            im.action_value.bind(action.clone(), 0.);
                        }
                    }
                    _ => clear(
                        &mut im,
                        (
                            &GamepadAxis::PositiveRightStickY,
                            &GamepadAxis::NegativeRightStickY,
                        ),
                    ),
                },
                GamepadAxisType::RightZ => todo!(),
                GamepadAxisType::Other(_) => todo!(),
            }
        }
    }
}
