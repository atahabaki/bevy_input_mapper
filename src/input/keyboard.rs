use bevy::prelude::*;

use crate::{AutoBinder, InputMapper};

impl<T> InputMapper<T>
where
    T: Copy + States,
{
    pub(crate) fn keyboard_key_press_system(
        mut im: ResMut<InputMapper<T>>,
        state: Res<State<T>>,
        input: Res<Input<KeyCode>>,
    ) {
        let im_iter = im.keyboard_binding.clone();
        let current_state = *state.get();
        for ((st, key), action) in im_iter.iter() {
            if st == &current_state {
                if input.pressed(*key) {
                    im.action_value.bind((current_state, action.to_owned()), 1.);
                }
                if input.just_released(*key) {
                    im.action_value.bind((current_state, action.to_owned()), 0.);
                }
            }
        }
    }
}
