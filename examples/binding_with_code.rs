use bevy::prelude::*;
use bevy_input_mapper::{
    input::{events::*, gamepad::GamepadAxis, mouse::MouseAxis},
    InputMapper, InputMapperPlugin,
};

fn bind_keys(mut im: ResMut<InputMapper>) {
    im.bind_keyboard_key_press(KeyCode::Space, "jump")
        .bind_gamepad_button_press(GamepadButtonType::South, "jump")
        .bind_mouse_axis_move(MouseAxis::PositiveX, "look_right")
        .bind_mouse_axis_move(MouseAxis::NegativeX, "look_left")
        .bind_mouse_axis_move(MouseAxis::PositiveY, "look_down")
        .bind_mouse_axis_move(MouseAxis::NegativeY, "look_up")
        .bind_gamepad_axis_move(GamepadAxis::PositiveRightStickX, "look_right")
        .bind_gamepad_axis_move(GamepadAxis::NegativeRightStickX, "look_left")
        .bind_gamepad_axis_move(GamepadAxis::NegativeRightStickY, "look_down")
        .bind_gamepad_axis_move(GamepadAxis::PositiveRightStickY, "look_up")
        .bind_mouse_button_press(MouseButton::Right, "scope")
        .bind_gamepad_button_press(GamepadButtonType::LeftTrigger, "scope")
        .bind_mouse_button_press(MouseButton::Left, "fire")
        .bind_gamepad_button_press(GamepadButtonType::RightTrigger2, "fire");
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
