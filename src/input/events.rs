use bevy::prelude::*;

use crate::{InputMapper, AutoBinder};

#[derive(Event)]
pub struct InputActionActive(pub String, pub f32);
#[derive(Event)]
pub struct InputActionStarted(pub String, pub f32);
#[derive(Event)]
pub struct InputActionContinuing(pub String, pub f32);
#[derive(Event)]
pub struct InputActionFinished(pub String);

impl InputMapper {
    pub fn event_cycle(
        mut im: ResMut<InputMapper>,
        mut action_active: EventWriter<InputActionActive>,
        mut action_started: EventWriter<InputActionStarted>,
        mut action_continuing: EventWriter<InputActionContinuing>,
        mut action_finished: EventWriter<InputActionFinished>,
    ) {
        let curr = im.action_value.clone();
        for (action, value) in curr.iter() {
            if im.is_active(&action) {
                action_active.send(InputActionActive(action.to_owned(), *value));
            }
            if im.is_started(&action) {
                action_started.send(InputActionStarted(action.to_owned(), *value));
            }
            if im.is_continuing(&action) {
                action_continuing.send(InputActionContinuing(action.to_owned(), *value));
            }
            if im.is_finished(&action) {
                action_finished.send(InputActionFinished(action.to_owned()));
            }
        }
        im.previous_action_value.clear();
        for (action, value) in curr.iter() {
            im.previous_action_value.bind(action.to_owned(), *value);
        }
        im.action_value.clear();
    }
}