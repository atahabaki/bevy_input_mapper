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
        let min_vec = -1. * Vec2::ONE;
        let max_vec = 1. * Vec2::ONE;
        let clamp_vec2 = |vector: Vec2| -> Vec2 {
            let clamp = |min: f32, max: f32, v: f32| -> f32 {
                if v < min {
                    min
                } else if v > max {
                    max
                } else {
                    v
                }
            };
            Vec2::new(clamp(min_vec.x, max_vec.x, vector.x), clamp(min_vec.y, max_vec.y, vector.y))
        };
        if let Some(motion) = mouse_motion.iter().last() {
            let normalized_delta = clamp_vec2(motion.delta);
            let axis_binding = im.mouse_axis_binding.clone();
            if normalized_delta.x > 0. {
                if let Some(action) = axis_binding.get(&MouseAxis::PositiveX) {
                    im.action_value.bind((*action).clone(), normalized_delta.x);
                }
            }
            if normalized_delta.x < 0. {
                if let Some(action) = axis_binding.get(&MouseAxis::NegativeX) {
                    im.action_value.bind((*action).clone(), normalized_delta.x.abs());
                }
            }
            if normalized_delta.y > 0. {
                if let Some(action) = axis_binding.get(&MouseAxis::PositiveY) {
                    im.action_value.bind((*action).clone(), normalized_delta.y);
                }
            }
            if normalized_delta.y < 0. {
                if let Some(action) = axis_binding.get(&MouseAxis::NegativeY) {
                    im.action_value.bind((*action).clone(), normalized_delta.y.abs());
                }
            }
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
