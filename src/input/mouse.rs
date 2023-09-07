use bevy::{prelude::*, input::mouse::MouseMotion};

use crate::{InputMapper, AutoBinder};

/// Represents a mouse's 2D movement axes.
/// Horizontal axis is X, Vertical axis is Y.
/// Default axis is PositiveX.
/// This axis system is simlar to the Screen Axis. You start counting from the top left corner.
/// Mouse movement is the same,
/// - Moving mouse to right is PositiveX,
/// - Moving mouse to left is NegativeX,
/// - Moving mouse to down is PositiveY,
/// - Moving mouse to up is NegativeY,
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum MouseAxis {
    /// Horizontal positive movement.
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

impl Default for MouseAxis {
    fn default() -> Self {
        MouseAxis::PositiveX
    }
}

impl InputMapper {
    pub fn mouse_axis_move_system(mut im: ResMut<InputMapper>, mut mouse_motion: EventReader<MouseMotion>) {
        let axis_binding = im.mouse_axis_binding.clone();
        let clear_x = |im: &mut ResMut<InputMapper>| {
            if let Some(action) = axis_binding.get(&MouseAxis::PositiveX) {
                im.action_value.bind((*action).clone(), 0.);
            }
            if let Some(action) = axis_binding.get(&MouseAxis::NegativeX) {
                im.action_value.bind((*action).clone(), 0.);
            }
        };
        let clear_y = |im: &mut ResMut<InputMapper>| {
            if let Some(action) = axis_binding.get(&MouseAxis::PositiveY) {
                im.action_value.bind((*action).clone(), 0.);
            }
            if let Some(action) = axis_binding.get(&MouseAxis::NegativeY) {
                im.action_value.bind((*action).clone(), 0.);
            }
        };
        let clear = |im: &mut ResMut<InputMapper>| {
            clear_x(im);
            clear_y(im);
        };
        if let Some(motion) = mouse_motion.iter().last() {
            // NOTE: Did `ö` got your attention? Be unusual when it comes to naming variables...
            match motion.delta.x {
                ö if ö > 0. => if let Some(action) = axis_binding.get(&MouseAxis::PositiveX) {
                    im.action_value.bind((*action).clone(), motion.delta.x);
                }
                ö if ö < 0. => if let Some(action) = axis_binding.get(&MouseAxis::NegativeX) {
                    im.action_value.bind((*action).clone(), motion.delta.x.abs());
                }
                _  => clear_x(&mut im),
            }
            match motion.delta.y {
                ö if ö > 0. => if let Some(action) = axis_binding.get(&MouseAxis::PositiveY) {
                    im.action_value.bind((*action).clone(), motion.delta.y);
                }
                ö if ö < 0. => if let Some(action) = axis_binding.get(&MouseAxis::NegativeY) {
                    im.action_value.bind((*action).clone(), motion.delta.y.abs());
                }
                _  => clear_y(&mut im),
            }
        } else {
            clear(&mut im);
        }
    }

    pub fn mouse_button_press_system(mut im: ResMut<InputMapper>, input: Res<Input<MouseButton>>) {
        let im_iter = im.mouse_button_binding.clone();
        for (button, action) in im_iter.iter() {
            if input.pressed(*button) {
                im.action_value.bind(action.to_owned(), 1.);
            }
            if input.just_released(*button) {
                im.action_value.bind(action.to_owned(), 0.);
            }
        }
    }
}
