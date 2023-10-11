use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{AutoBinder, InputMapper};

/// Represents a mouse's 2D movement axes.
/// Horizontal axis is X, Vertical axis is Y.
/// Default axis is PositiveX.
/// This axis system is simlar to the Screen Axis. You start counting from the top left corner.
/// Mouse movement is the same,
/// - Moving mouse to right is PositiveX,
/// - Moving mouse to left is NegativeX,
/// - Moving mouse to down is PositiveY,
/// - Moving mouse to up is NegativeY,
#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub enum MouseAxis {
    /// Horizontal positive movement.
    #[default]
    PositiveX,
    /// Horizontal negative movement.
    NegativeX,
    /// Reversed Vertical positive movement.
    /// Imagine, moving your mouse top right corner to bottom right corner.
    PositiveY,
    /// Reversed Vertical negative movement.
    /// Imagine, moving your mouse bottom right corner to top right corner.
    NegativeY,
}

impl<T> InputMapper<T>
where
    T: Copy + States,
{
    pub(crate) fn mouse_axis_move_system(
        mut im: ResMut<InputMapper<T>>,
        state: Res<State<T>>,
        mut mouse_motion: EventReader<MouseMotion>,
    ) {
        let axis_binding = im.mouse_axis_binding.clone();
        let current_state = *state.get();
        let clear_x = |im: &mut ResMut<InputMapper<T>>| {
            if let Some(action) = axis_binding.get(&(current_state, MouseAxis::PositiveX)) {
                im.action_value.bind((current_state, (*action).clone()), 0.);
            }
            if let Some(action) = axis_binding.get(&(current_state, MouseAxis::NegativeX)) {
                im.action_value.bind((current_state, (*action).clone()), 0.);
            }
        };
        let clear_y = |im: &mut ResMut<InputMapper<T>>| {
            if let Some(action) = axis_binding.get(&(current_state, MouseAxis::PositiveY)) {
                im.action_value.bind((current_state, (*action).clone()), 0.);
            }
            if let Some(action) = axis_binding.get(&(current_state, MouseAxis::NegativeY)) {
                im.action_value.bind((current_state, (*action).clone()), 0.);
            }
        };
        let clear = |im: &mut ResMut<InputMapper<T>>| {
            clear_x(im);
            clear_y(im);
        };
        if let Some(motion) = mouse_motion.iter().last() {
            // NOTE: Did `ö` got your attention? Be unusual when it comes to naming variables...
            match motion.delta.x {
                ö if ö > 0. => {
                    if let Some(action) = axis_binding.get(&(current_state, MouseAxis::PositiveX)) {
                        im.action_value
                            .bind((current_state, (*action).clone()), motion.delta.x);
                    }
                }
                ö if ö < 0. => {
                    if let Some(action) = axis_binding.get(&(current_state, MouseAxis::NegativeX)) {
                        im.action_value
                            .bind((current_state, (*action).clone()), motion.delta.x.abs());
                    }
                }
                _ => clear_x(&mut im),
            }
            match motion.delta.y {
                ö if ö > 0. => {
                    if let Some(action) = axis_binding.get(&(current_state, MouseAxis::PositiveY)) {
                        im.action_value
                            .bind((current_state, (*action).clone()), motion.delta.y);
                    }
                }
                ö if ö < 0. => {
                    if let Some(action) = axis_binding.get(&(current_state, MouseAxis::NegativeY)) {
                        im.action_value
                            .bind((current_state, (*action).clone()), motion.delta.y.abs());
                    }
                }
                _ => clear_y(&mut im),
            }
        } else {
            clear(&mut im);
        }
    }

    pub(crate) fn mouse_button_press_system(
        mut im: ResMut<InputMapper<T>>,
        state: Res<State<T>>,
        input: Res<Input<MouseButton>>,
    ) {
        let im_iter = im.mouse_button_binding.clone();
        let current_state = *state.get();
        for ((st, button), action) in im_iter.iter() {
            if st == &current_state {
                if input.pressed(*button) {
                    im.action_value.bind((current_state, action.to_owned()), 1.);
                }
                if input.just_released(*button) {
                    im.action_value.bind((current_state, action.to_owned()), 0.);
                }
            }
        }
    }
}
