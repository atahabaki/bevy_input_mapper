use bevy::prelude::*;
use bevy_input_mapper::{
    input::{events::*, gamepad::GamepadAxis, mouse::MouseAxis},
    AutoBinder, InputMapper, InputMapperPlugin,
};

fn bind_keys(mut im: ResMut<InputMapper>) {
    im.keyboard_binding.bind(KeyCode::Space, "jump".to_string());
    im.mouse_button_binding
        .bind(MouseButton::Left, "fire".to_string());
    im.mouse_axis_binding
        .bind(MouseAxis::PositiveX, "look_right".to_string());
    im.mouse_axis_binding
        .bind(MouseAxis::NegativeY, "look_up".to_string());
    im.mouse_axis_binding
        .bind(MouseAxis::NegativeX, "look_left".to_string());
    im.mouse_axis_binding
        .bind(MouseAxis::PositiveY, "look_down".to_string());
    im.gamepad_axis_binding
        .bind(GamepadAxis::PositiveRightStickY, "look_up".to_string());
    im.gamepad_axis_binding
        .bind(GamepadAxis::NegativeRightStickY, "look_down".to_string());
    im.gamepad_axis_binding
        .bind(GamepadAxis::PositiveRightStickX, "look_right".to_string());
    im.gamepad_axis_binding
        .bind(GamepadAxis::NegativeRightStickX, "look_left".to_string());
    im.gamepad_button_binding
        .bind(GamepadButtonType::LeftTrigger, "scope".to_string());
    im.gamepad_button_binding
        .bind(GamepadButtonType::RightTrigger2, "fire".to_string());
    im.gamepad_button_binding
        .bind(GamepadButtonType::South, "jump".to_string());
}

fn logger(
    mut action_active: EventReader<InputActionActive>,
    mut action_started: EventReader<InputActionStarted>,
    mut action_continuing: EventReader<InputActionContinuing>,
    mut action_finished: EventReader<InputActionFinished>,
) {
    for ev in action_active.iter() {
        info!("Action Active: {}, {}", ev.0, ev.1);
    }
    for ev in action_started.iter() {
        info!("Action Started: {}, {}", ev.0, ev.1);
    }
    for ev in action_continuing.iter() {
        info!("Action Continuing: {}, {}", ev.0, ev.1);
    }
    for ev in action_finished.iter() {
        info!("Action Finished: {}", ev.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputMapperPlugin)
        .add_systems(Startup, bind_keys)
        .add_systems(Update, logger)
        .run()
}
