use bevy::prelude::*;
use bevy_input_mapper::{
    input::{events::*, gamepad::GamepadAxis, mouse::MouseAxis},
    InputMapper, InputMapperPlugin,
};

/// Here, we define a State for Scenario.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Default,
    Swimming,
}

/// We bind Input with specific scenario to an action.
fn bind_keys(mut im: ResMut<InputMapper<GameState>>) {
    // On default Scenario, pressing Space or Gamepad South triggers jump action.
    im.bind_keyboard_key_press(GameState::Default, KeyCode::Space, "jump")
        .bind_gamepad_button_press(GameState::Default, GamepadButtonType::South, "jump")
        // On swimming Scenario/State, pressing Space or Gamepad South triggers swim_up action.
        .bind_keyboard_key_press(GameState::Swimming, KeyCode::Space, "swim_up")
        .bind_gamepad_button_press(GameState::Swimming, GamepadButtonType::South, "swim_up")
        // Here we bind gamepad's right stick and mouse movements to camera.
        .bind_gamepad_axis_move(
            GameState::Default,
            GamepadAxis::NegativeRightStickX,
            "look_left",
        )
        .bind_gamepad_axis_move(
            GameState::Default,
            GamepadAxis::PositiveRightStickX,
            "look_right",
        )
        .bind_gamepad_axis_move(
            GameState::Default,
            GamepadAxis::NegativeRightStickY,
            "look_down",
        )
        .bind_gamepad_axis_move(
            GameState::Default,
            GamepadAxis::PositiveRightStickY,
            "look_up",
        )
        .bind_mouse_axis_move(GameState::Default, MouseAxis::NegativeX, "look_left")
        .bind_mouse_axis_move(GameState::Default, MouseAxis::PositiveX, "look_right")
        .bind_mouse_axis_move(GameState::Default, MouseAxis::PositiveY, "look_down")
        .bind_mouse_axis_move(GameState::Default, MouseAxis::NegativeY, "look_up");
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
        .add_state::<GameState>()
        .add_plugins(InputMapperPlugin::<GameState>::new())
        .add_systems(Startup, bind_keys)
        .add_systems(Update, logger)
        .run()
}
