use bevy::prelude::*;

use crate::{InputMapper, AutoBinder};

impl InputMapper {
    pub fn keyboard_key_press_bind(
        &mut self,
        action: &str,
        input: KeyCode
    ) {
        self.keyboard_binding.bind(input, action.to_owned());
    }
    pub fn keyboard_key_press_system(mut im: ResMut<InputMapper>, input: Res<Input<KeyCode>>) {
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
