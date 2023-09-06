use bevy::prelude::*;

use crate::{InputMapper, AutoBinder};

/// Represents a mouse's 2D movement axes.
/// Horizontal axis is X, Vertical axis is Y.
/// Default axis is PositiveX.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum MouseAxis {
    /// Horizontal positive movement.
    PositiveX,
    /// Horizontal negative movement.
    NegativeX,
    /// Vertical positive movement.
    PositiveY,
    /// Vertical negative movement.
    NegativeY,
}

impl Default for MouseAxis {
    fn default() -> Self {
        MouseAxis::PositiveX
    }
}

impl InputMapper {
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
