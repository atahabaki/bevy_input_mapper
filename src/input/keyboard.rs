use bevy::prelude::*;

use crate::{AutoBinder, InputMapper};

impl InputMapper {
    pub(crate) fn keyboard_key_press_system(
        mut im: ResMut<InputMapper>,
        input: Res<Input<KeyCode>>,
    ) {
        let im_iter = im.keyboard_binding.clone();
        for (key, action) in im_iter.iter() {
            if input.pressed(*key) {
                im.action_value.bind(action.to_owned(), 1.);
            }
            if input.just_released(*key) {
                im.action_value.bind(action.to_owned(), 0.);
            }
        }
    }
}
