use bevy::{
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput},
    prelude::*,
    utils::HashMap,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_input_mapper::{InputMapper, InputMapperPlugin};

/// Here, we define a State for Scenario.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Default,
    Swimming,
}

#[derive(Default, Resource)]
pub struct UiState {
    /// action name -> button name
    button_names: HashMap<String, String>,
    /// If the button pressed for re-mapping.
    is_listening: bool,
    /// If the button pressed for re-mapping, which one?
    which_action_listening: String,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(InputMapperPlugin::<GameState>::new())
        .add_state::<GameState>()
        .init_resource::<UiState>()
        .add_systems(Startup, bind_keys)
        .add_systems(Update, remapper)
        .run();
}

fn bind_keys(mut im: ResMut<InputMapper<GameState>>) {
    // On default Scenario, pressing Space or Gamepad South triggers jump action.
    im.bind_keyboard_key_press(GameState::Default, KeyCode::Space, "jump")
        .bind_gamepad_button_press(GameState::Default, GamepadButtonType::South, "jump")
        // On swimming Scenario/State, pressing Space or Gamepad South triggers swim_up action.
        .bind_keyboard_key_press(GameState::Swimming, KeyCode::Space, "swim_up")
        .bind_gamepad_button_press(GameState::Swimming, GamepadButtonType::South, "swim_up");
}

fn remapper(
    mut ctx: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut keyboard: EventReader<KeyboardInput>,
    mut mouse: EventReader<MouseButtonInput>,
    mut im: ResMut<InputMapper<GameState>>,
) {
    egui::Window::new("Keyboard Actions (Default Scenario)").show(ctx.ctx_mut(), |ui| {
        if ui_state.is_listening {
            for ev in mouse.iter() {
                im.bind_mouse_button_press(
                    GameState::Default,
                    ev.button,
                    ui_state.which_action_listening.clone(),
                );
                ui_state.is_listening = false;
                ui_state.which_action_listening = String::new();
            }
            for ev in keyboard.iter() {
                if let Some(code) = ev.key_code {
                    im.bind_keyboard_key_press(
                        GameState::Default,
                        code,
                        ui_state.which_action_listening.clone(),
                    );
                }
                ui_state.is_listening = false;
                ui_state.which_action_listening = String::new();
            }
        }
        for action in im.list_scenario_actions_keyboard_mouse(GameState::Default) {
            if ui_state.which_action_listening == action {
                ui_state
                    .button_names
                    .insert(action.clone(), format!("Listening {}", action.clone()));
            } else {
                ui_state
                    .button_names
                    .insert(action.clone(), format!("Bind {}", action.clone()));
            }

            if ui
                .button(ui_state.button_names.get(&action.clone()).unwrap())
                .clicked()
            {
                ui_state.is_listening = true;
                ui_state.which_action_listening = action;
            }
        }
    });
    egui::Window::new("Keyboard Actions (Swimming Scenario)").show(ctx.ctx_mut(), |ui| {
        for action in im.list_scenario_actions_keyboard_mouse(GameState::Swimming) {
            if ui.button(format!("Bind {action}")).clicked() {}
        }
    });
}
