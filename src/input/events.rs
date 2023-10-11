use bevy::prelude::*;

use crate::{AutoBinder, InputMapper};

#[derive(Event)]
pub struct InputActionActive(pub String, pub f32);
#[derive(Event)]
pub struct InputActionStarted(pub String, pub f32);
#[derive(Event)]
pub struct InputActionContinuing(pub String, pub f32);
#[derive(Event)]
pub struct InputActionFinished(pub String);

impl<T> InputMapper<T>
where
    T: Copy + States,
{
    pub(crate) fn event_cycle(
        mut im: ResMut<InputMapper<T>>,
        mut action_active: EventWriter<InputActionActive>,
        mut action_started: EventWriter<InputActionStarted>,
        mut action_continuing: EventWriter<InputActionContinuing>,
        mut action_finished: EventWriter<InputActionFinished>,
    ) {
        let curr = im.action_value.clone();
        for (action, value) in curr.iter() {
            if im.is_active(action.0, &action.1) {
                action_active.send(InputActionActive(action.1.to_owned(), *value));
            }
            if im.is_started(action.0, &action.1) {
                action_started.send(InputActionStarted(action.1.to_owned(), *value));
            }
            if im.is_continuing(action.0, &action.1) {
                action_continuing.send(InputActionContinuing(action.1.to_owned(), *value));
            }
            if im.is_finished(action.0, &action.1) {
                action_finished.send(InputActionFinished(action.1.to_owned()));
            }
        }
        for (action, value) in curr.iter() {
            im.previous_action_value.bind(action.to_owned(), *value);
        }
    }
}
